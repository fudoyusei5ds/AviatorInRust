pub const VS_STR: &str = r#"
#version 330 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

layout(std140) uniform MyBlock {
    mat4 view;
    mat4 perspective;
    mat4 lightView;
    mat4 lightPerspective;
};

uniform vec3 object_color;
uniform mat4 model;

out VS_OUT {
    vec3 FragPos;           // 普通空间下点的坐标
    vec3 Normal;            // 点的法线
    vec4 FragPosLightSpace; // 光照空间下点的坐标
    vec3 Color;             // 物体的颜色
} vs_out;

out float y;

void main() {
    vs_out.Color = object_color;
    y = (position.y + 1)/2;

    gl_Position = perspective*view*model*vec4(position, 1.0);

    vs_out.FragPos = vec3(model * vec4(position, 1.0f));
    vs_out.Normal = mat3(transpose(inverse(model)))*normal;
    vs_out.FragPosLightSpace = lightPerspective*lightView*vec4(vs_out.FragPos, 1.0);
}
"#;

pub const SHADOW_VS_STR: &str = r#"
#version 330 core
layout (location = 0) in vec3 position;

layout(std140) uniform MyBlock {
    mat4 view;
    mat4 perspective;
};

uniform mat4 model;

void main()
{
    gl_Position = perspective * view * model * vec4(position, 1.0f);
}
"#;