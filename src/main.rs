#[macro_use]
extern crate glium;
use glium::Surface;

mod cylinder;
mod plane;
mod camera;
mod screen;

mod shade_fs;
mod shade_vs;

fn main() {
    // 创建事件循环
    let mut events_loop = glium::glutin::EventsLoop::new();
    // 创建窗口
    let window = glium::glutin::WindowBuilder::new()
                    .with_dimensions(glium::glutin::dpi::LogicalSize::new(800.0, 600.0))
                    .with_title("aviator");
    // 创建上下文
    let context = glium::glutin::ContextBuilder::new()
                    .with_depth_buffer(24)
                    .with_srgb(true)
                    .with_multisampling(4);
    // 创建显示
    let display = glium::backend::glutin::Display::new(window, context, &events_loop).unwrap();

    // 创建着色器程序
    let sourcecode = glium::program::ProgramCreationInput::SourceCode {
        vertex_shader: shade_vs::vs_str,
        tessellation_control_shader: None,
        tessellation_evaluation_shader: None,
        geometry_shader: None,
        fragment_shader: shade_fs::fs_str,
        transform_feedback_varyings: None,
        outputs_srgb: true,
        uses_point_size: true,
    };
    let program = glium::Program::new(&display, sourcecode).unwrap();

    // 创建场景
    let mut airplane = plane::Plane::new(&display);
    airplane.set_scale(0.2, 0.2, 0.2);
    let mut sea = cylinder::Cylinder::new(&display);
    sea.set_scale(8.0, 8.0, 8.0);
    sea.set_position(0.0, -9.0, 0.0);

    // 创建镜头
    let view_camera = camera::Camera::new(&[0.0, 1.0, -2.0], &[0.0, -1.0, 2.0]);

    // 创建屏幕
    let initscreen = screen::Screen::new(&display);

    let mut closed = false;
    while !closed {
        // 创建frame
        let mut target= display.draw();
        // 清理背景颜色
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        // 创建帧缓冲
        let color_texture = glium::texture::srgb_texture2d_multisample::SrgbTexture2dMultisample::empty(&display, 800, 600, 4).unwrap();
        let depth_texture = glium::texture::depth_texture2d_multisample::DepthTexture2dMultisample::empty(&display, 800, 600, 4).unwrap();
        
        let mut frame_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &color_texture, &depth_texture).unwrap();
        frame_buffer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // 绘制场景到新建帧缓冲
        airplane.draw(&mut frame_buffer, &program, &view_camera.view, &view_camera.perspective);
        sea.wave(&display);
        sea.draw(&mut frame_buffer, &program, &view_camera.view, &view_camera.perspective);

        // 把纹理绘制到屏幕上
        initscreen.draw(&mut target, &color_texture);

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