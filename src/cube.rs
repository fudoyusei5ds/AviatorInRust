#[path="geom.rs"]
mod geom;

use glium::Surface;

pub struct Cube {
    vertex_buffer: glium::VertexBuffer<geom::Position>,       // 顶点缓冲
    normal_buffer: glium::VertexBuffer<geom::Normal>,         // 顶点缓冲, 保存每个顶点的法线向量 
    index_buffer: glium::IndexBuffer<u16>,              // 索引缓冲
    color: [f32; 3],                                    // 颜色
    position: [[f32;4];4],                      // 位置坐标矩阵
    rotate: [[f32;4];4],                        // 旋转矩阵
    scale: [[f32;4];4],                         // 尺寸矩阵
    pmodel: [[f32;4];4],                        // 父节点模型矩阵
}

impl Cube {
    // 这个函数用来新建一个立方体对象
    pub fn new(display: &glium::Display) -> Cube {
        let shape = vec![
            // 前
            geom::Position {position: [-0.5, 0.5, -0.5]},  // 0
            geom::Position {position: [0.5, 0.5, -0.5]},   // 1
            geom::Position {position: [0.5, -0.5, -0.5]},  // 2
            geom::Position {position: [-0.5, -0.5, -0.5]},   // 3
            // 左
            geom::Position {position: [0.5, 0.5, -0.5]},   // 4
            geom::Position {position: [0.5, 0.5, 0.5]},   // 5
            geom::Position {position: [0.5, -0.5, 0.5]},   // 6
            geom::Position {position: [0.5, -0.5, -0.5]},  // 7
            // 后
            geom::Position {position: [0.5, 0.5, 0.5]},   // 8
            geom::Position {position: [-0.5, 0.5, 0.5]},   // 9
            geom::Position {position: [-0.5, -0.5, 0.5]},   // 10
            geom::Position {position: [0.5, -0.5, 0.5]},   // 11
            // 右
            geom::Position {position: [-0.5, 0.5, 0.5]},   // 12
            geom::Position {position: [-0.5, 0.5, -0.5]},  // 13
            geom::Position {position: [-0.5, -0.5, -0.5]},   // 14
            geom::Position {position: [-0.5, -0.5, 0.5]},   // 15
            // 上
            geom::Position {position: [-0.5, 0.5, 0.5]},   // 16
            geom::Position {position: [0.5, 0.5, 0.5]},   // 17
            geom::Position {position: [0.5, 0.5, -0.5]},   // 18
            geom::Position {position: [-0.5, 0.5, -0.5]},  // 19
            // 下
            geom::Position {position: [-0.5, -0.5, -0.5]},   // 20
            geom::Position {position: [0.5, -0.5, -0.5]},  // 21
            geom::Position {position: [-0.5, -0.5, 0.5]},   // 22
            geom::Position {position: [0.5, -0.5, 0.5]},   // 23
        ];
        // 立方体每个面上的各个顶点的法线向量其实是一样的.  
        let mut normals: Vec<geom::Normal> = Vec::new();
        for index in 0..24 {
            if index < 4 {
                normals.push(geom::Normal {normal: [0.0, 0.0, -1.0]});
            }
            else if index < 8 {
                normals.push(geom::Normal {normal: [1.0, 0.0, 0.0]});
            }
            else if index < 12 {
                normals.push(geom::Normal {normal: [0.0, 0.0, 1.0]});
            }
            else if index < 16 {
                normals.push(geom::Normal {normal: [-1.0, 0.0, 0.0]});
            }
            else if index < 20 {
                normals.push(geom::Normal {normal: [0.0, 1.0, 0.0]});
            }
            else {
                normals.push(geom::Normal {normal: [0.0, -1.0, 0.0]});
            }
        }
        // 统一初始化为单位矩阵
        let initmatrix: [[f32; 4]; 4] = [ [1.0, 0.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [0.0, 0.0, 0.0, 1.0f32]]; 
        return Cube {
            vertex_buffer: glium::VertexBuffer::new(display, &shape).unwrap(),
            normal_buffer: glium::VertexBuffer::new(display, &normals).unwrap(),
            index_buffer: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                &[0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8,12,13,14,14,15,12,16,17,18,18,19,16,20,21,22,22,23,20,],).unwrap(),
            position: initmatrix,
            rotate: initmatrix,
            scale: initmatrix, 
            pmodel: initmatrix,
            color: [1.0, 1.0, 1.0f32],
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
        // 创建绘制参数
        let mut params: glium::draw_parameters::DrawParameters = Default::default();
        params.depth = glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLessOrEqual,
            write: true,
            ..Default::default()
        };
        params.multisampling = true;
        target.draw(
                    (&self.vertex_buffer, &self.normal_buffer),
                    &self.index_buffer,
                    program, 
                    &uniforms,
                    &params
        ).unwrap();
    }

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