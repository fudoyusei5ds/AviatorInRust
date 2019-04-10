pub const FS_STR: &str = r#"
#version 330
out vec4 FragColor;

uniform sampler2D shadowMap;

in VS_OUT {
    vec3 FragPos;           // 普通空间下点的坐标
    vec3 Normal;            // 点的法线
    vec4 FragPosLightSpace; // 光照空间下点的坐标
    vec3 Color;             // 物体的颜色
} fs_in;
in float y;

// 在这里设置光线的方向和颜色
vec3 lightDir = normalize(vec3(2.0, 2.0, 0.0));
vec3 lightColor = vec3(1.0, 1.0, 1.0);

float ShadowCalculation(vec4 fragPosLightSpace) {
    // 执行透视除法
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;

    projCoords = projCoords * 0.5 + 0.5;

    float closestDepth = texture(shadowMap, projCoords.xy).r;

    float currentDepth = projCoords.z;

    float shadow = currentDepth > closestDepth  ? 1.0 : 0.0;

    return shadow;
}

void main() {
    // 半球光
    vec3 hemisphere = vec3(y, y, y) * 0.69 * 0.9;

    // 漫反射光
    vec3 norm = normalize(fs_in.Normal);
    float diff = max(dot(norm, lightDir), 0.0); // 计算漫反射光的强度
    vec3 diffuse = diff * lightColor;

    // 计算阴影
    float shadow = ShadowCalculation(fs_in.FragPosLightSpace);

    // 输出颜色
    vec3 result = (hemisphere + (1.0 - shadow)) * diffuse * fs_in.Color;
    FragColor = vec4(result, 1.0);
}
"#;

pub const SHADOW_FS_STR: &str = r#"
#version 330 core

void main()
{             
    // gl_FragDepth = gl_FragCoord.z;
}
"#;