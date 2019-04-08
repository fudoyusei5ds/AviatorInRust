pub const vs_str: &str = r#"
#version 330

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

uniform vec3 object_color;
uniform mat4 view;
uniform mat4 perspective;
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