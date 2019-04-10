#[macro_use]
extern crate glium;
use glium::Surface;

mod cylinder;
mod plane;
mod camera;
mod screen;

mod shade_fs;
mod shade_vs;

#[derive(Copy, Clone)]
struct Block {
    view: [[f32; 4]; 4],
    perspective: [[f32; 4]; 4]
}
implement_uniform_block! (Block, view, perspective);

#[derive(Copy, Clone)]
struct ShadowBlock {
    view: [[f32; 4]; 4],
    perspective: [[f32; 4]; 4],
    lightView: [[f32; 4]; 4],
    lightPerspective: [[f32; 4]; 4]
}
implement_uniform_block! (ShadowBlock, view, perspective, lightView, lightPerspective);

fn main() {
    // 创建事件循环
    let mut events_loop = glium::glutin::EventsLoop::new();

    // 创建窗口
    let monitor = glium::glutin::Window::new(&events_loop).unwrap();
    let window = glium::glutin::WindowBuilder::new()
                    .with_dimensions(glium::glutin::dpi::PhysicalSize::new(800.0, 600.0)
                        .to_logical(monitor.get_hidpi_factor()))
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
        vertex_shader: shade_vs::VS_STR,
        tessellation_control_shader: None,
        tessellation_evaluation_shader: None,
        geometry_shader: None,
        fragment_shader: shade_fs::FS_STR,
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

    // 创建阴影的视角
    let shadow_camera = camera::Camera::new(&[2.0, 2.0, 0.0], &[-2.0, -2.0, 0.0]);
    // 创建阴影的着色器程序
    let shadow_program = glium::Program::from_source(
        &display, 
        shade_vs::SHADOW_VS_STR, 
        shade_fs::SHADOW_FS_STR, 
        None).unwrap();

    let mut closed = false;
    while !closed {
        // 动画
        sea.wave(&display);

        // 创建frame
        let mut target= display.draw();
        // 清理背景颜色
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        // 创建帧缓冲用来保存阴影
        let shadow_depth_texture = 
            glium::texture::depth_texture2d::DepthTexture2d::empty(&display, 1024, 1024).unwrap();
        let shadow_color_texture = 
            glium::texture::texture2d::Texture2d::empty(&display, 1024, 1024).unwrap();

        // 创建一个只有深度缓冲的帧缓冲
        let mut shadow_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &shadow_color_texture, &shadow_depth_texture).unwrap();
        shadow_buffer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        // 在深度贴图中渲染场景
        let shadow_uniform_block = glium::uniforms::UniformBuffer::new(
            &display,
            Block {
                view: shadow_camera.view,
                perspective: shadow_camera.perspective,
            }).unwrap();
        airplane.draw(&mut shadow_buffer, &shadow_program, &shadow_uniform_block, &glium::texture::depth_texture2d::DepthTexture2d::empty(&display, 1024, 1024).unwrap());
        sea.draw(&mut shadow_buffer, &shadow_program, &shadow_uniform_block, &glium::texture::depth_texture2d::DepthTexture2d::empty(&display, 1024, 1024).unwrap());

        // 创建帧缓冲
        let color_texture = glium::texture::srgb_texture2d_multisample::SrgbTexture2dMultisample::empty(&display, 800, 600, 4).unwrap();
        let depth_texture = glium::texture::depth_texture2d_multisample::DepthTexture2dMultisample::empty(&display, 800, 600, 4).unwrap();
        
        let mut frame_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &color_texture, &depth_texture).unwrap();
        frame_buffer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // 创建一个uniform缓冲
        let uniform_block = glium::uniforms::UniformBuffer::new(
            &display, ShadowBlock {
                view: view_camera.view,
                perspective: view_camera.perspective,
                lightView: shadow_camera.view,
                lightPerspective: shadow_camera.perspective,
            }).unwrap();

        // 绘制场景到新建帧缓冲
        airplane.draw(&mut frame_buffer, &program, &uniform_block, &shadow_depth_texture);
        sea.draw(&mut frame_buffer, &program, &uniform_block, &shadow_depth_texture);

        // 将帧缓冲的内容绘制到默认帧缓冲中
        target.blit_from_simple_framebuffer(
            &frame_buffer, 
            &glium::Rect{left:0, bottom: 0, width: 800, height: 600}, 
            &glium::BlitTarget{left:0, bottom: 0, width: 800, height: 600}, 
            glium::uniforms::MagnifySamplerFilter::Nearest);

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