pub const fs_str: &str = r#"
#version 330
in vec3 v_color;
in vec3 v_normal;
in float y;

out vec4 color;

// 在这里设置光线的方向和颜色
vec3 lightDir = normalize(vec3(2.0, 2.0, 0.0));
vec3 lightColor = vec3(1.0, 1.0, 1.0);

void main() {
    // 环境光
    float ambientStrength = 0.2; // 设置环境光的强度
    vec3 ambient = ambientStrength * lightColor;

    // 半球光
    vec3 hemisphere = vec3(y, y, y) * 0.69 * 0.9;

    // 漫反射光
    vec3 norm = normalize(v_normal);
    float diff = max(dot(norm, lightDir), 0.0); // 计算漫反射光的强度
    vec3 diffuse = diff * lightColor;

    // 输出颜色
    vec3 result = (hemisphere + diffuse) * v_color;
    color = vec4(result, 1.0);
}
"#;