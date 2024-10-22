// src/distance_joint.rs

use bevy::prelude::*;
use crate::cuboid::Cuboid;


#[derive(Component)]
pub struct DistanceJoint {
    pub body_a: Entity, // 连接的第一个实体
    pub body_b: Entity, // 连接的第二个实体
    pub min_distance: f32, // 最小距离
    pub max_distance: f32, // 最大距离
}

impl DistanceJoint {
    pub fn new(body_a: Entity, body_b: Entity, min_distance: f32, max_distance: f32) -> Self {
        Self {
            body_a,
            body_b,
            min_distance,
            max_distance,
        }
    }
}
