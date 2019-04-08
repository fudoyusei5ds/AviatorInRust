pub const VS_SRC: &str = r#"
#version 330
layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texcoord;

out vec2 v_tex_coords;

void main() {
    v_tex_coords = texcoord;
    gl_Position = vec4(position, 1.0);
}
"#;

pub const FS_SRC: &str = r#"
#version 330

in vec2 v_tex_coords;
out vec4 FragColor;

uniform sampler2D screen_texture;

void main() {
    FragColor = texture2D(screen_texture, v_tex_coords);
}
"#;

// 顶点位置
#[derive(Copy, Clone)]
struct Vertex{
    position: [f32; 3],
    texcoord: [f32; 2],
}
implement_vertex!(Vertex, position, texcoord);

pub struct Screen {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u32>,
    program: glium::Program
}

impl Screen {
    pub fn new(display: &glium::Display) -> Screen {
        let shape = vec! [
            Vertex {position: [-1.0, 1.0, 0.0], texcoord: [0.0, 1.0]},
            Vertex {position: [1.0, 1.0, 0.0], texcoord: [1.0, 1.0]},
            Vertex {position: [1.0, -1.0, 0.0], texcoord: [1.0, 0.0]},
            Vertex {position: [-1.0, -1.0, 0.0], texcoord: [0.0, 0.0]},
        ];
        let indexs = vec! [
            0, 1, 2,
            2, 3, 0,
        ];
        Screen {
            vertex_buffer: glium::VertexBuffer::new(display, &shape).unwrap(),
            index_buffer: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList , &indexs).unwrap(),
            program: glium::Program::from_source(display, VS_SRC, FS_SRC, None).unwrap(),
        }
    }

    pub fn draw<S>(&self, 
        target: &mut S, 
        texture: &glium::texture::srgb_texture2d_multisample::SrgbTexture2dMultisample)
    where
        S: glium::Surface
    {
        target.draw(&self.vertex_buffer, 
            &self.index_buffer,
            &self.program, 
            &uniform!{
                screen_texture: 
                    texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)}, 
            &Default::default()).unwrap();
    }
}