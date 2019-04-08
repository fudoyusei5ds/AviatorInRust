pub struct Camera {
    pub view: [[f32; 4]; 4],
    pub perspective: [[f32; 4]; 4],
}

impl Camera {
    pub fn new(position: &[f32; 3], direction: &[f32; 3]) -> Camera{
        return Camera {
            view: view_matrix(position, direction, &[0.0, 1.0, 0.0]),
            perspective: {
                let aspect_ratio: f32 = 0.75;
                let fov: f32 = 3.141592/3.0;
                let zfar = 1024.0;
                let znear = 0.1;
                let f = 1.0/(fov/2.0).tan();
                [
                    [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                    [         0.0         ,     f ,              0.0              ,   0.0],
                    [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                    [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                ]
            }
        }
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