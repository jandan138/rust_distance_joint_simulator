//src\cuboid.rs

use bevy::prelude::*;
use bevy::ecs::bundle::Bundle;

// 定义Cuboid结构体
#[derive(Component)]
pub struct Cuboid {
    pub position: Vec3,
    pub size: Vec3,
    pub mass: f32,
}

impl Cuboid {
    // 添加构造函数
    pub fn new(size: Vec3, position: Vec3, mass: f32) -> Self {
        Self { size, position, mass }
    }
}

// CuboidBundle包括一个PbrBundle和Cuboid组件
#[derive(Bundle)]
pub struct CuboidBundle {
    #[bundle]
    pub pbr_bundle: PbrBundle,
    pub cuboid: Cuboid,
}

impl CuboidBundle {
    pub fn new(cuboid: Cuboid, mesh: Handle<Mesh>, material: Handle<StandardMaterial>, transform: Transform) -> Self {
        Self {
            pbr_bundle: PbrBundle {
                mesh,
                material,
                transform,
                ..Default::default()
            },
            cuboid,
        }
    }
}
