// src/rope_constraint.rs

use bevy::prelude::*;
use crate::cuboid::Cuboid;
use crate::physics::PhysicsBody;

#[derive(Component)]
pub struct RopeConstraint {
    pub physics_body_a: PhysicsBody, // 连接的第一个实体
    pub physics_body_b: PhysicsBody, // 连接的第二个实体
    //pub min_distance: f32, // 最小距离
    pub max_distance: f32, // 绳子最大长度
}

//设置全局的绳子长度约束数组
#[derive(Resource)]
struct RopeConstraints {
    constraints: Vec<RopeConstraint>,
}

impl RopeConstraint {
    pub fn new(physics_body_a: PhysicsBody, physics_body_b: PhysicsBody, max_distance: f32) -> Self {
        Self {
            physics_body_a,
            physics_body_b,
            max_distance,
        }
    }
}

// 维持距离约束的系统
pub fn maintain_rope_constraints(mut constraints: ResMut<RopeConstraints>) 
{
    //遍历一个全局RopeConstraint的数组 对于里面每个RopeConstraint，如果他们距离physics_body_a, physics_body_b 的predicted_position的差值超过绳子最大长度 ，
    //修改其中的每个            physics_body_a, physics_body_b的velocity, 把他们沿绳子方向的速度变为零
    let mut physics_body_a = constraints.physics_body_a;
    let mut physics_body_b = constraints.physics_body_b;
    
    let direction = (physics_body_a.predicted_position - physics_body_a.predicted_position).normalize();//b->a的单位向量

    physics_a.cancel_velocity_along(direction);
    physics_b.cancel_velocity_along(direction);
    // 应用所有修正

}



