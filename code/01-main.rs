#[macro_use]
extern crate glium;
use glium::Surface;

#[derive(Copy, Clone)]
pub struct Position{
    pub position: [f32; 3]
}
implement_vertex!(Position, position);

pub struct Cube {
    vertex_buffer: glium::VertexBuffer<Position>, // 顶点缓冲
    index_buffer: glium::IndexBuffer<u16>,              // 索引缓冲
    color: [f32; 3],                                    // 颜色
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
        return Cube {
            vertex_buffer: glium::VertexBuffer::new(display, &shape).unwrap(),
            index_buffer: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                &[0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8,12,13,14,14,15,12,16,17,18,18,19,16,20,21,22,22,23,20,],).unwrap(),
            color: [1.0, 0.0, 0.0f32],
        }
    }

    // 绘制函数
    pub fn draw(&self,
        target: &mut glium::Frame, 
        program: &glium::Program,
        view: &[[f32; 4]; 4],
        perspective: &[[f32; 4]; 4],)
    {
        let uniforms = uniform! {
            object_color: self.color, 
            view: *view,
            perspective: *perspective,
        };
        target.draw(
                    &self.vertex_buffer,
                    &self.index_buffer,
                    program, 
                    &uniforms,
                    &Default::default()
        ).unwrap();
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

        uniform vec3 object_color;
        uniform mat4 view;
        uniform mat4 perspective;

        out vec3 color;

        void main() {
            color = object_color;
            gl_Position = perspective * view * vec4(position, 1.0);
        }
    "#;
    let fs_str: &str = r#"
        #version 330
        in vec3 color;
        out vec4 frag_color;

        void main() {
            frag_color = vec4(color, 1.0);
        }
    "#;

    // 创建着色器程序
    let program = glium::Program::from_source(&display, vs_str, fs_str, None).unwrap();
    // 创建一个立方体
    let newcube = Cube::new(&display);

    let mut closed = false;
    while !closed {
        // 创建帧缓冲
        let mut target= display.draw();
        // 清理背景颜色
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        // 添加观察矩阵
        let view = view_matrix(&[1.0, 1.0, -1.0], &[-1.0, -1.0, 1.0], &[1.0, 0.0, 0.0]);
        // 添加透视矩阵
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
        newcube.draw(&mut target, &program, &view, &perspective);
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