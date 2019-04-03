#[macro_use]
extern crate glium;
use glium::Surface;

extern crate cgmath;

#[derive(Copy, Clone)]
pub struct Position{
    pub position: [f32; 3]
}
implement_vertex!(Position, position);

// 法线向量
#[derive(Copy, Clone)]
pub struct Normal{
    pub normal: [f32; 3]
}
implement_vertex!(Normal, normal);

pub struct Cube {
    vertex_buffer: glium::VertexBuffer<Position>,       // 顶点缓冲
    normal_buffer: glium::VertexBuffer<Normal>,         // 顶点缓冲, 保存每个顶点的法线向量 
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
            Position {position: [-0.5, 0.5, -0.5]},  // 0
            Position {position: [0.5, 0.5, -0.5]},   // 1
            Position {position: [0.5, -0.5, -0.5]},  // 2
            Position {position: [-0.5, -0.5, -0.5]},   // 3
            // 左
            Position {position: [0.5, 0.5, -0.5]},   // 4
            Position {position: [0.5, 0.5, 0.5]},   // 5
            Position {position: [0.5, -0.5, 0.5]},   // 6
            Position {position: [0.5, -0.5, -0.5]},  // 7
            // 后
            Position {position: [0.5, 0.5, 0.5]},   // 8
            Position {position: [-0.5, 0.5, 0.5]},   // 9
            Position {position: [-0.5, -0.5, 0.5]},   // 10
            Position {position: [0.5, -0.5, 0.5]},   // 11
            // 右
            Position {position: [-0.5, 0.5, 0.5]},   // 12
            Position {position: [-0.5, 0.5, -0.5]},  // 13
            Position {position: [-0.5, -0.5, -0.5]},   // 14
            Position {position: [-0.5, -0.5, 0.5]},   // 15
            // 上
            Position {position: [-0.5, 0.5, 0.5]},   // 16
            Position {position: [0.5, 0.5, 0.5]},   // 17
            Position {position: [0.5, 0.5, -0.5]},   // 18
            Position {position: [-0.5, 0.5, -0.5]},  // 19
            // 下
            Position {position: [-0.5, -0.5, -0.5]},   // 20
            Position {position: [0.5, -0.5, -0.5]},  // 21
            Position {position: [-0.5, -0.5, 0.5]},   // 22
            Position {position: [0.5, -0.5, 0.5]},   // 23
        ];
        // 立方体每个面上的各个顶点的法线向量其实是一样的.  
        let mut normals: Vec<Normal> = Vec::new();
        for index in 0..24 {
            if index < 4 {
                normals.push(Normal {normal: [0.0, 0.0, -1.0]});
            }
            else if index < 8 {
                normals.push(Normal {normal: [1.0, 0.0, 0.0]});
            }
            else if index < 12 {
                normals.push(Normal {normal: [0.0, 0.0, 1.0]});
            }
            else if index < 16 {
                normals.push(Normal {normal: [-1.0, 0.0, 0.0]});
            }
            else if index < 20 {
                normals.push(Normal {normal: [0.0, 1.0, 0.0]});
            }
            else {
                normals.push(Normal {normal: [0.0, -1.0, 0.0]});
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
    pub fn draw(&self,
        target: &mut glium::Frame, 
        program: &glium::Program,
        view: &[[f32; 4]; 4],
        perspective: &[[f32; 4]; 4],)
    {
        let model: [[f32; 4]; 4] = 
            matrix_multi(&self.scale, &matrix_multi(&self.rotate, &matrix_multi(&self.position, &self.pmodel)));
        let uniforms = uniform! {
            object_color: self.color, 
            view: *view,
            perspective: *perspective,
            model: model,
        };
        // 创建绘制参数
        let mut params: glium::draw_parameters::DrawParameters = Default::default();
        params.depth = glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLessOrEqual,
            write: true,
            ..Default::default()
        };
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

pub struct Plane {
    wing: Cube,         // 机翼
    cockpit: Cube,      // 机舱
    engine: Cube,       // 引擎
    propeller: Cube,    // 螺旋浆
    matblade: Cube,     // 叶片
    tail: Cube,         // 机尾
    position: [[f32; 4]; 4],
    rotate: [[f32; 4]; 4],
    scale: [[f32; 4]; 4],
}

impl Plane {
    // 初始化函数
    pub fn new(display: &glium::Display) -> Plane {
        // 设置飞机的各个部件
        // 设置机舱
        let mut cockpit = Cube::new(display);
        cockpit.set_scale(1.2, 1.0, 1.0);
        cockpit.set_color(0.95, 0.33, 0.27);
        // 设置引擎
        let mut engine = Cube::new(display);
        engine.set_scale(0.4, 1.0, 1.0);
        engine.set_position(0.8, 0.0, 0.0);
        engine.set_color(0.85, 0.82, 0.82);
        // 设置机尾
        let mut tail = Cube::new(display);
        tail.set_scale(0.3, 0.4, 0.1);
        tail.set_position(-0.7, 0.5, 0.0);
        tail.set_color(0.95, 0.33, 0.27);
        // 设置机翼
        let mut wing = Cube::new(display);
        wing.set_scale(0.8, 0.16, 3.0);
        wing.set_color(0.95, 0.33, 0.27);
        // 设置螺旋浆
        let mut propeller = Cube::new(display);
        propeller.set_scale(0.4, 0.2, 0.2);
        propeller.set_position(1.2, 0.0, 0.0);
        propeller.set_color(0.35, 0.20, 0.18);
        // 设置叶片
        let mut matblade = Cube::new(display);
        matblade.set_scale(0.02, 2.0, 0.4);
        matblade.set_position(1.2, 0.0, 0.0);
        matblade.set_color(0.14, 0.10, 0.06);

        let initmatrix = [[1.0, 0.0, 0.0, 0.0],
                          [0.0, 1.0, 0.0, 0.0],
                          [0.0, 0.0, 1.0, 0.0],
                          [0.0, 0.0, 0.0, 1.0f32]];
        Plane {
            cockpit, engine, tail, wing, propeller, matblade, 
            position: initmatrix,
            rotate: initmatrix,
            scale: initmatrix,
        }
    }

    // 绘制函数
    pub fn draw(&mut self, 
        target: &mut glium::Frame, 
        program: &glium::Program,
        view: &[[f32; 4]; 4],
        perspective: &[[f32; 4]; 4],) 
    {
        let model: [[f32; 4]; 4] = 
            matrix_multi(&self.scale, &matrix_multi(&self.rotate, &self.position));
        self.wing.set_pmodel(&model); 
        self.wing.draw(target, program, view, perspective);
        self.cockpit.set_pmodel(&model);
        self.cockpit.draw(target, program, view, perspective);
        self.engine.set_pmodel(&model);
        self.engine.draw(target, program, view, perspective);
        self.tail.set_pmodel(&model);
        self.tail.draw(target, program, view, perspective);
        self.propeller.set_pmodel(&model);
        self.propeller.draw(target, program, view, perspective);
        self.matblade.set_pmodel(&model);
        self.matblade.draw(target, program, view, perspective);
    }

    // 设置位置
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = [[1.0, 0.0, 0.0, 0.0],
             [0.0, 1.0, 0.0, 0.0],
             [0.0, 0.0, 1.0, 0.0],
             [x, y, z, 1.0f32]];
    }
    // 设置尺寸
    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = [
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ]
    }
    // 设置旋转角度
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
}


fn main() {
    // 创建事件循环
    let mut events_loop = glium::glutin::EventsLoop::new();
    // 创建窗口
    let window = glium::glutin::WindowBuilder::new()
                    .with_dimensions(glium::glutin::dpi::LogicalSize::new(800.0, 600.0))
                    .with_title("aviator");
    // 创建上下文
    let context = glium::glutin::ContextBuilder::new();
    // 创建显示
    let display = glium::backend::glutin::Display::new(window, context, &events_loop).unwrap();

    // 着色器代码
    let vs_str: &str = r#"
        #version 330

        layout(location = 0) in vec3 position;
        layout(location = 1) in vec3 normal;

        uniform vec3 object_color;
        uniform mat4 view;
        uniform mat4 perspective;
        uniform mat4 model;

        out vec3 v_color;
        out vec3 v_normal;

        void main() {
            v_color = object_color;
            v_normal = mat3(transpose(inverse(model))) * normal;

            gl_Position = perspective*view*model*vec4(position, 1.0);
        }
    "#;
    let fs_str: &str = r#"
        #version 330
        in vec3 v_color;
        in vec3 v_normal;

        out vec4 color;

        // 在这里设置光线的方向和颜色
        vec3 lightDir = normalize(vec3(1.0, 2.0, 0.0));
        vec3 lightColor = vec3(1.0, 1.0, 1.0);

        void main() {
            // 环境光
            float ambientStrength = 0.4; // 设置环境光的强度
            vec3 ambient = ambientStrength * lightColor;

            // 漫反射光
            vec3 norm = normalize(v_normal);
            float diff = max(dot(norm, lightDir), 0.0); // 计算漫反射光的强度
            vec3 diffuse = diff * lightColor;

            // 输出颜色
            vec3 result = (ambient + diffuse) * v_color;
            color = vec4(result, 1.0);
        }
    "#;

    // 创建着色器程序
    let program = glium::Program::from_source(&display, vs_str, fs_str, None).unwrap();
    // 创建一个立方体
    let mut fly = Plane::new(&display);

    let mut closed = false;

    while !closed {
        // 创建frame
        let mut target= display.draw();
        // 清理背景颜色
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        // 添加透视
        let view = view_matrix(&[0.0, 10.0, -10.0], &[0.0, -10.0, 10.0], &[0.0, 1.0, 0.0]);
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        // 绘制立方体
        fly.draw(&mut target, &program, &view, &perspective);
        // 将帧缓冲绘制到屏幕上
        target.finish().unwrap();
        // 事件循环
        events_loop.poll_events(|ev| {
            match ev {
                glium::glutin::Event::WindowEvent {event, ..} => match event {
                    glium::glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    // 首先求方向向量的单位向量
    let f = {
        let f = direction;
        let len = f[0]*f[0] + f[1]*f[1] + f[2]*f[2];
        let len = len.sqrt();
        [f[0]/len, f[1]/len, f[2]/len]
    };
    // 计算左向量
    let s = [
        up[1]*f[2] - up[2]*f[1],
        up[2]*f[0] - up[0]*f[2],
        up[0]*f[1] - up[1]*f[0],
    ];
    let s_norm = {
        let len = s[0]*s[0] + s[1]*s[1] + s[2]*s[2];
        let len = len.sqrt();
        [s[0]/len, s[1]/len, s[2]/len]
    };
    // 计算方向向量与左向量的叉乘, 即上向量
    let u = [
        f[1]*s_norm[2] - f[2]*s_norm[1],
        f[2]*s_norm[0] - f[0]*s_norm[2],
        f[0]*s_norm[1] - f[1]*s_norm[0],
    ];
    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]
    ];

    return [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

// 矩阵乘法
fn matrix_multi(first: &[[f32; 4]; 4], second: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut a: [[f32; 4]; 4] = [[0.0; 4]; 4];
    for i in 0..4 {
        for k in 0..4 {
            let r:f32 = first[i][k];
            for j in 0..4 {
                a[i][j] += r*second[k][j];
            }
        }
    }
    return a;
}