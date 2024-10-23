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

// 维持距离约束的系统
pub fn maintain_distance_joints(
    mut commands: Commands,
    query: Query<(Entity, &DistanceJoint, &Transform)>
) {
    for (entity, joint, _transform) in query.iter() {
        let transform_a = query.get_component::<Transform>(joint.body_a).unwrap();
        let transform_b = query.get_component::<Transform>(joint.body_b).unwrap();

        let current_distance = transform_a.translation.distance(transform_b.translation);
        if current_distance < joint.min_distance || current_distance > joint.max_distance {
            commands.entity(entity).insert(Transform::default());
        }
    }
}
