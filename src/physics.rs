// src/physics.rs
use bevy::prelude::*;
use bevy::ecs::system::Res;

// PhysicsBody 组件包含速度、加速度和质量
#[derive(Component)]
pub struct PhysicsBody {
    pub velocity: Vec3, // 速度
    pub acceleration: Vec3, // 加速度
    pub mass: f32, // 质量
}

impl PhysicsBody {
    // 创建一个新的 PhysicsBody 实例，通常用于初始化
    pub fn new(mass: f32) -> Self {
        Self {
            velocity: Vec3::ZERO, // 初始速度为零
            acceleration: Vec3::ZERO, // 初始加速度为零
            mass,
        }
    }
}

// physics_step_system 系统负责更新所有物理体的状态
pub fn physics_step_system(
    time: Res<Time>, // 访问游戏时间，用于计算时间步长
    mut query: Query<(&mut Transform, &mut PhysicsBody)>, // 查询所有具有 Transform 和 PhysicsBody 的实体
) {
    for (mut transform, mut physics) in query.iter_mut() {
        // 使用欧拉积分更新速度和位置
        physics.velocity += physics.acceleration * time.delta_seconds();
        transform.translation += physics.velocity * time.delta_seconds();

        // 重置加速度（如果每帧计算加速度）
        physics.acceleration = Vec3::ZERO;
    }
}
