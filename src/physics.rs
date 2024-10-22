//src\physics.rs

use bevy::prelude::*;
use bevy::ecs::system::Res;

// 定义重力向量，假设重力作用在Z轴负方向
const GRAVITY: Vec3 = Vec3::new(0.0, 0.0, -9.81);

// PhysicsBody 组件存储速度、力和质量
#[derive(Component)]
pub struct PhysicsBody {
    pub velocity: Vec3,       // 当前速度
    pub force: Vec3,          // 当前作用的总力
    pub mass: f32,            // 质量
}

impl PhysicsBody {
    // 创建一个新的 PhysicsBody 实例
    pub fn new(mass: f32) -> Self {
        Self {
            velocity: Vec3::ZERO,           // 初始化速度为零
            force: Vec3::ZERO,              // 初始化力为零
            mass,
        }
    }

    // 向物体添加力
    pub fn apply_force(&mut self, force: Vec3) {
        self.force += force;
    }

    // 计算并更新物理状态
    pub fn update_physics(&mut self, delta_time: f32) {
        let acceleration = self.force / self.mass; // F = ma, a = F/m
        self.velocity += acceleration * delta_time;
        self.force = Vec3::ZERO; // 重置力，准备下一帧的计算
    }
}

// 物理步进系统更新每个物体的状态
pub fn physics_step_system(
    time: Res<Time>, 
    mut query: Query<(&mut Transform, &mut PhysicsBody)>
) {
    for (mut transform, mut physics) in query.iter_mut() {
        let mass = physics.mass;  // 先复制质量到局部变量
        physics.apply_force(GRAVITY * mass);  // 使用局部变量计算新力
        physics.update_physics(time.delta_seconds()); // 更新物理状态

        transform.translation += physics.velocity * time.delta_seconds();
    }
}

