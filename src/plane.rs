#[path="cube.rs"]
mod cube;

#[path="geom.rs"]
mod geom;

pub struct Plane {
    wing: cube::Cube,         // 机翼
    cockpit: cube::Cube,      // 机舱
    engine: cube::Cube,       // 引擎
    propeller: cube::Cube,    // 螺旋浆
    matblade: cube::Cube,     // 叶片
    tail: cube::Cube,         // 机尾
    position: [[f32; 4]; 4],
    rotate: [[f32; 4]; 4],
    scale: [[f32; 4]; 4],
}

impl Plane {
    // 初始化函数
    pub fn new(display: &glium::Display) -> Plane {
        // 设置飞机的各个部件
        // 设置机舱
        let mut cockpit = cube::Cube::new(display);
        cockpit.set_scale(1.2, 1.0, 1.0);
        cockpit.set_color(0.95, 0.33, 0.27);
        // 设置引擎
        let mut engine = cube::Cube::new(display);
        engine.set_scale(0.4, 1.0, 1.0);
        engine.set_position(0.8, 0.0, 0.0);
        engine.set_color(0.85, 0.82, 0.82);
        // 设置机尾
        let mut tail = cube::Cube::new(display);
        tail.set_scale(0.3, 0.4, 0.1);
        tail.set_position(-0.7, 0.5, 0.0);
        tail.set_color(0.95, 0.33, 0.27);
        // 设置机翼
        let mut wing = cube::Cube::new(display);
        wing.set_scale(0.8, 0.16, 3.0);
        wing.set_color(0.95, 0.33, 0.27);
        // 设置螺旋浆
        let mut propeller = cube::Cube::new(display);
        propeller.set_scale(0.4, 0.2, 0.2);
        propeller.set_position(1.2, 0.0, 0.0);
        propeller.set_color(0.35, 0.20, 0.18);
        // 设置叶片
        let mut matblade = cube::Cube::new(display);
        matblade.set_scale(0.02, 2.0, 0.4);
        matblade.set_position(1.2, 0.0, 0.0);
        matblade.set_color(0.14, 0.10, 0.06);

        let initmatrix = [[1.0, 0.0, 0.0, 0.0],
                          [0.0, 1.0, 0.0, 0.0],
                          [0.0, 0.0, 1.0, 0.0],
                          [0.0, 0.0, 0.0, 1.0f32]];
        Plane {
            cockpit, engine, tail, wing, propeller, matblade, 
            position: initmatrix,
            rotate: initmatrix,
            scale: initmatrix,
        }
    }

    // 绘制函数
    pub fn draw<S, T>(&mut self, 
        target: &mut S, 
        program: &glium::Program,
        uniform: &glium::uniforms::UniformBuffer<T>,
        depth: &glium::texture::depth_texture2d::DepthTexture2d)
    where
        S: glium::Surface, 
        T: glium::uniforms::UniformBlock+glium::buffer::Content,
    {
        let model: [[f32; 4]; 4] = 
            geom::matrix_multi(&self.scale, 
                &geom::matrix_multi(&self.rotate, &self.position));
        self.wing.set_pmodel(&model); 
        self.wing.draw(target, program, uniform, depth);
        self.cockpit.set_pmodel(&model);
        self.cockpit.draw(target, program, uniform, depth);
        self.engine.set_pmodel(&model);
        self.engine.draw(target, program, uniform, depth);
        self.tail.set_pmodel(&model);
        self.tail.draw(target, program, uniform,depth);
        self.propeller.set_pmodel(&model);
        self.propeller.draw(target, program, uniform,depth);
        self.matblade.set_pmodel(&model);
        self.matblade.draw(target, program, uniform,depth);
    }

    // 设置位置
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = [[1.0, 0.0, 0.0, 0.0],
             [0.0, 1.0, 0.0, 0.0],
             [0.0, 0.0, 1.0, 0.0],
             [x, y, z, 1.0f32]];
    }
    // 设置尺寸
    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = [
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ]
    }
    // 设置旋转角度
    pub fn set_rotate(&mut self, angle: f32, xyz: i32) {
        self.rotate = if xyz==0 {
            // 沿x轴旋转
            [[1.0, 0.0, 0.0, 0.0],
             [0.0, angle.cos(), angle.sin(), 0.0],
             [0.0, -angle.sin(), angle.cos(), 0.0],
             [0.0, 0.0, 0.0, 1.0]]
        } else if xyz==1 {
            // 绕y轴旋转
            [[angle.cos(), 0.0, -angle.sin(), 0.0],
             [0.0, 1.0, 0.0, 0.0],
             [angle.sin(), 0.0, angle.cos(), 0.0],
             [0.0, 0.0, 0.0, 1.0]]
        } else if xyz==2 {
            // 绕z轴旋转
            [[angle.cos(), angle.sin(), 0.0, 0.0],
             [-angle.sin(), angle.cos(), 0.0, 0.0],
             [0.0, 0.0, 1.0, 0.0],
             [0.0, 0.0, 0.0, 1.0]]
        } else {
            [[1.0, 0.0, 0.0, 0.0],
             [0.0, 1.0, 0.0, 0.0],
             [0.0, 0.0, 1.0, 0.0],
             [0.0, 0.0, 0.0, 1.0]]
        };
    }
}