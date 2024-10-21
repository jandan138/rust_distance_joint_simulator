use bevy::prelude::*;

// 定义一个代表立方体的结构体
#[derive(Component)]
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
        // 简单的位移更新逻辑
        let acceleration = force / self.mass;
        let velocity = acceleration * delta_time;
        self.position += velocity;
    }
}

#[derive(Bundle)]
pub struct CuboidBundle {
    pub cuboid: Cuboid,
    #[bundle]
    pub transform: TransformBundle,
    pub material: Handle<StandardMaterial>,
    pub mesh: Handle<Mesh>,
}
