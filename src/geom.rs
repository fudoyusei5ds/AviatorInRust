// 顶点坐标
#[derive(Copy, Clone)]
pub struct Position{
    pub position: [f32; 3]
}
implement_vertex!(Position, position);

// 法线向量
#[derive(Copy, Clone)]
pub struct Normal{
    pub normal: [f32; 3]
}
implement_vertex!(Normal, normal);

// 矩阵乘法
pub fn matrix_multi(first: &[[f32; 4]; 4], second: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut a: [[f32; 4]; 4] = [[0.0; 4]; 4];
    for i in 0..4 {
        for k in 0..4 {
            let r:f32 = first[i][k];
            for j in 0..4 {
                a[i][j] += r*second[k][j];
            }
        }
    }
    return a;
}