pub const VS_STR: &str = r#"
#version 330

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

layout(std140) uniform MyBlock {
    mat4 view;
    mat4 perspective;
}

uniform vec3 object_color;
uniform mat4 model;

out vec3 v_color;
out vec3 v_normal;
out float y;

void main() {
    v_color = object_color;
    v_normal = mat3(transpose(inverse(model)))*normal;
    y = (position.y + 1)/2;

    gl_Position = perspective*view*model*vec4(position, 1.0);
}
"#;

pub const SHADOW_VS_STR: &str = r#"
#version 330 core
layout (location = 0) in vec3 position;

uniform mat4 view;
uniform mat4 perspective;
uniform mat4 model;

void main()
{
    gl_Position = lightSpaceMatrix * model * vec4(position, 1.0f);
}
"#;