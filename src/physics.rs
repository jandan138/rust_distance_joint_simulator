//src\physics.rs

use bevy::prelude::*;
use bevy::ecs::system::Res;

// 定义重力向量，假设重力作用在Z轴负方向
const GRAVITY: Vec3 = Vec3::new(0.0, 0.0, -1.1);

// PhysicsBody 组件存储速度、力和质量
#[derive(Component)]
pub struct PhysicsBody {
    pub velocity: Vec3,       // 当前速度
    pub force: Vec3,          // 当前作用的总力
    pub mass: f32,            // 质量
    pub is_fixed: bool,  // 新增属性，确定是否固定位置
}

impl PhysicsBody {
    // 创建一个新的 PhysicsBody 实例
    pub fn new(mass: f32) -> Self {
        Self {
            velocity: Vec3::ZERO,           // 初始化速度为零
            force: Vec3::ZERO,              // 初始化力为零
            mass,
            is_fixed : false,
        }
    }
    // 创建一个新的 PhysicsBody 实例，带有指定的初始速度
    pub fn new_with_velocity(mass: f32, initial_velocity: Vec3) -> Self {
        Self {
            velocity: initial_velocity,
            force: Vec3::ZERO,
            mass,
            is_fixed : false,  // 设置是否固定
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

    // 清除沿指定方向的速度分量
    pub fn cancel_velocity_along(&mut self, direction: Vec3) {
        let velocity_along = self.velocity.dot(direction) * direction;
        self.velocity -= velocity_along;
    }    
}

// 物理步进系统更新每个物体的状态
pub fn physics_step_system(
    time: Res<Time>, 
    mut query: Query<(Entity, &mut Transform, &mut PhysicsBody)>
) {
    for (entity, mut transform, mut physics) in query.iter_mut() {
        let mass = physics.mass;  // 先复制质量到局部变量
        println!("更新前的Entity: {:?}, Position: {:?}, Velocity: {:?}, Mass: {}", entity, transform.translation, physics.velocity, mass);
        physics.apply_force(GRAVITY * mass);  // 使用局部变量计算新力
        physics.update_physics(time.delta_seconds()); // 更新物理状态

        // 在应用物理计算后更新位置
        let delta_transform = physics.velocity * time.delta_seconds() ;
        // 仅当 is_fixed 为 false 时更新位置
        if !physics.is_fixed {
            transform.translation += physics.velocity * time.delta_seconds();
        }

        // 输出调试信息
        println!("更新后的Entity: {:?}, Position: {:?}, Velocity: {:?}, Mass: {}, 时间的变化量：{}, 位置的变化量:{:?}", entity, transform.translation, physics.velocity, mass, time.delta_seconds(), delta_transform);
    }
}

