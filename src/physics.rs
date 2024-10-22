use bevy::prelude::*;
use bevy::ecs::system::Res;

// 定义重力向量，假设重力作用在Z轴负方向
const GRAVITY: Vec3 = Vec3::new(0.0, 0.0, -9.81);

// PhysicsBody 组件存储速度、加速度和质量
#[derive(Component)]
pub struct PhysicsBody {
    pub velocity: Vec3,       // 当前速度
    pub acceleration: Vec3,   // 当前加速度
    pub mass: f32,            // 质量
}

impl PhysicsBody {
    // 创建一个新的 PhysicsBody 实例
    pub fn new(mass: f32) -> Self {
        Self {
            velocity: Vec3::ZERO,           // 初始化速度为零
            acceleration: Vec3::ZERO,       // 初始化加速度为零
            mass,
        }
    }
}

// 物理步进系统更新每个物体的状态
pub fn physics_step_system(
    time: Res<Time>, 
    mut query: Query<(&mut Transform, &mut PhysicsBody)>
) {
    for (mut transform, mut physics) in query.iter_mut() {
        let mass = physics.mass;  // 先复制质量到局部变量
        physics.acceleration += GRAVITY * mass;  // 使用局部变量计算新加速度

        let acceleration = physics.acceleration; // 再复制加速度到另一个局部变量
        physics.velocity += acceleration * time.delta_seconds();
        transform.translation += physics.velocity * time.delta_seconds();

        physics.acceleration = Vec3::ZERO;
    }
}
