extern crate rand;

#[path="geom.rs"]
mod geom;

use glium::Surface;

pub struct Cylinder {
    vbo: (glium::VertexBuffer<geom::Position>, glium::VertexBuffer<geom::Normal>), // 顶点缓冲
    waves: [[f32; 3]; 440],
    vertex: Vec<geom::Position>,
    position: [[f32;4];4],                      // 位置坐标矩阵
    rotate: [[f32;4];4],                        // 旋转矩阵
    scale: [[f32;4];4],                         // 尺寸矩阵
    pmodel: [[f32;4];4],                        // 父节点模型矩阵
    color: [f32; 3],
}

impl Cylinder {
    pub fn wave(&mut self, display: &glium::Display) {
        let mut new_vertex: Vec<geom::Position> = Vec::new();
        for index in 0..440 {
            let x: f32 = self.vertex[index].position[0];
            let x: f32 = x + self.waves[index][0].cos() * self.waves[index][1];
            let y: f32 = self.vertex[index].position[1];
            let y: f32 = y + self.waves[index][0].sin() * self.waves[index][1];
            let z: f32 = self.vertex[index].position[2];
            new_vertex.push(geom::Position{position:[x, y, z]});
            self.waves[index][0] += self.waves[index][2];
        }
        self.vbo = Cylinder::create_vbo(display, &new_vertex);
    }
    pub fn create_vbo(display: &glium::Display, vertex: &Vec<geom::Position>) -> (glium::VertexBuffer<geom::Position>, glium::VertexBuffer<geom::Normal>) {
        let mut shape: Vec<geom::Position> = Vec::new();
        for i in 0..40 {
            for index in 0..10 {
                let first = index + i*11;
                if i==39 {
                    // 正面
                    shape.push(vertex[index+1]);
                    shape.push(vertex[first+1]);
                    shape.push(vertex[first]);
                    // 反面
                    shape.push(vertex[first]);
                    shape.push(vertex[index]);
                    shape.push(vertex[index+1]);
                }
                else {
                    // 正面
                    shape.push(vertex[first+12]);
                    shape.push(vertex[first+1]);
                    shape.push(vertex[first]);
                    // 反面
                    shape.push(vertex[first]);
                    shape.push(vertex[first+11]);
                    shape.push(vertex[first+12]);
                }
            }
        }
        let mut normals: Vec<geom::Normal> = Vec::new();
        // 一共600个三角形
        for i in 0..800 {
            // 每个三角形3个顶点
            let a:[f32; 3] = [shape[i*3+1].position[0]-shape[i*3].position[0], shape[i*3+1].position[1]-shape[i*3].position[1], shape[i*3+1].position[2]-shape[i*3].position[2]];
            let b:[f32; 3] = [shape[i*3+2].position[0]-shape[i*3+1].position[0], shape[i*3+2].position[1]-shape[i*3+1].position[1], shape[i*3+2].position[2]-shape[i*3+1].position[2]];
            // 求个叉乘
            let normal:[f32; 3] = [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]];
            for _j in 0..3 {
                normals.push(geom::Normal {normal});
            }
        }
        (   glium::VertexBuffer::new(display, &shape).unwrap(), 
            glium::VertexBuffer::new(display, &normals).unwrap())
    }
    // 新建对象
    pub fn new(display: &glium::Display) -> Cylinder {
        // 1. 确定所有顶点的坐标, 并按顺序排列好
        let mut vertex: Vec<geom::Position> = Vec::new();
        let mut index = 0;
        let mut waves:[[f32; 3]; 440]=[[0.0; 3]; 440];
        for i in 0..40 {
            let angle: f32 = std::f32::consts::PI / 20.0 * i as f32;
            for z in -5..6 {
                let x: f32 = angle.cos();
                let y: f32 = angle.sin();
                vertex.push(geom::Position {position: [x, y, 0.1 * z as f32]});
                waves[index] = [
                    rand::random::<f32>() * std::f32::consts::PI * 2.0,     // 随机角度
                    0.01 + rand::random::<f32>() * 0.03,                    // 随机距离
                    0.016 +  rand::random::<f32>() * 0.032                  // 转动角度
                ];
                index+=1;
            }
        }
        let vbo = Cylinder::create_vbo(display, &vertex);
        // 4. 生成圆柱体
        let initmatrix: [[f32; 4]; 4] = [ [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0f32]];
        Cylinder {
            vbo,
            vertex,
            waves,
            position: initmatrix,
            rotate: initmatrix,
            scale: initmatrix,
            pmodel: initmatrix,
            color: [0.41, 0.76, 0.76f32],
        }
    }

    // 绘制函数
    pub fn draw<S, T>(&self,
        target: &mut S, 
        program: &glium::Program, 
        uniform: &glium::uniforms::UniformBuffer<T>,
        depth: &glium::texture::depth_texture2d::DepthTexture2d)
    where
        S: glium::Surface, 
        T: glium::uniforms::UniformBlock+glium::buffer::Content,
    {
        // 开启深度测试
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        let model: [[f32; 4]; 4] = 
            geom::matrix_multi(&self.scale, 
                &geom::matrix_multi(&self.rotate, 
                    &geom::matrix_multi(&self.position, &self.pmodel)));
        
        let uniforms = uniform! {
            object_color: self.color, 
            MyBlock: uniform,
            model: model,
            shadowMap: depth,
        };
        
        target.draw(
                (&self.vbo.0, &self.vbo.1),
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                program, 
                &uniforms, 
                &params
        ).unwrap();
    }

    // 设置父节点模型矩阵
    pub fn set_pmodel(&mut self, model: &[[f32; 4]; 4]) {
        self.pmodel = *model;
    } 

    // 对于转换来说, 首先要进行缩放操作
    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = [
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ]
    }

    // 然后是旋转
    pub fn set_rotate(&mut self, angle: f32, xyz: i32) {
        self.rotate = if xyz==0 {
            // 沿x轴旋转
            [[1.0, 0.0, 0.0, 0.0],
                [0.0, angle.cos(), angle.sin(), 0.0],
                [0.0, -angle.sin(), angle.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]]
        } else if xyz==1 {
            // 绕y轴旋转
            [[angle.cos(), 0.0, -angle.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [angle.sin(), 0.0, angle.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]]
        } else if xyz==2 {
            // 绕z轴旋转
            [[angle.cos(), angle.sin(), 0.0, 0.0],
                [-angle.sin(), angle.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]]
        } else {
            [[1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]]
        };
    }

    // 最后进行位移操作
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = [[1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0f32]];
    }

    // 设置颜色
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }
}