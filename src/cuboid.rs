pub struct Cuboid {
    pub position: Vec3,
    pub size: Vec3,
    pub mass: f32,
}

impl Cuboid {
    pub fn new(size: Vec3, position: Vec3, mass: f32) -> Self {
        Self { size, position, mass }
    }

    pub fn update(&mut self, force: Vec3, delta_time: f32) {
        // 更新位置等逻辑
    }
}
