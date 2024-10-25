// src/distance_joint.rs

use bevy::prelude::*;
use crate::cuboid::Cuboid;
use crate::physics::PhysicsBody;

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
    query: Query<(Entity, &DistanceJoint, &Transform, &PhysicsBody)>,
    transform_query: Query<&Transform>,
) {
    let mut corrections: Vec<(Entity, Vec3)> = Vec::new();

    // 遍历所有包含 DistanceJoint、Transform 和 PhysicsBody 的实体
    for (entity, joint, transform, physics) in query.iter() {
        // 使用 transform_query 分别获取 body_a 和 body_b 的 Transform 组件
        let transform_a = transform_query.get(joint.body_a);
        let transform_b = transform_query.get(joint.body_b);

        // 确保两个 Transform 都能够获取到，才继续处理逻辑
        if let (Ok(transform_a), Ok(transform_b)) = (transform_a, transform_b) {
            // 获取到 body_a 和 body_b 的 Transform 后进行距离判断
            let current_distance = transform_a.translation.distance(transform_b.translation);
            println!("当前距离: {}", current_distance);  // 输出当前两个实体之间的距离
            println!("设定的最小距离: {}", joint.min_distance);  // 输出设定的最小距离
            println!("设定的最大距离: {}", joint.max_distance);  // 输出设定的最大距离

            // 如果当前距离超出设定的范围，进行校正
            if current_distance < joint.min_distance || current_distance > joint.max_distance {
                let correction = (transform_b.translation - transform_a.translation).normalize()
                    * (current_distance - (joint.min_distance + joint.max_distance) / 2.0);
                println!("应用的校正: {}", correction);  // 输出应用的位置校正向量

                corrections.push((joint.body_a, transform_a.translation + correction));
                corrections.push((joint.body_b, transform_b.translation - correction));
            }
        }
    }

    // 应用所有修正
    for (entity, new_translation) in corrections {
        // 输出每个 entity 的信息
        println!("应用修正到实体: {:?}, 新的位置: {:?}", entity, new_translation);
        
        commands.entity(entity).insert(Transform {
            translation: new_translation,
            ..Default::default()
        });
    }

}














// pub fn maintain_distance_joints(
//     mut commands: Commands,
//     query: Query<(Entity, &DistanceJoint, &Transform, &PhysicsBody)>
// ) {
//     let mut corrections: Vec<(Entity, Vec3)> = Vec::new();

//     for (entity, joint, transform, physics) in query.iter() {
//         let transform_a = query.get_component::<Transform>(joint.body_a).unwrap();
//         let transform_b = query.get_component::<Transform>(joint.body_b).unwrap();

//         //let current_distance = transform_a.translation.distance(transform_b.translation);
//         //println!("当前距离: {}", current_distance);  // 输出当前两个实体之间的距离
//         println!("设定的最小距离: {}", joint.min_distance);  // 输出设定的最小距离
//         println!("设定的最大距离: {}", joint.max_distance);  // 输出设定的最大距离

//         // if current_distance < joint.min_distance || current_distance > joint.max_distance {
//         //     let correction = (transform_b.translation - transform_a.translation).normalize() * (current_distance - (joint.min_distance + joint.max_distance) / 2.0);
//         //     println!("应用的校正: {}", correction);  // 输出应用的位置校正向量

//         //     corrections.push((joint.body_a, transform_a.translation - correction));
//         //     corrections.push((joint.body_b, transform_b.translation + correction));
//         // }
//     }

//     // 应用所有修正
//     // for (entity, new_translation) in corrections {
//     //     commands.entity(entity).insert(Transform {
//     //         translation: new_translation,
//     //         ..Default::default()
//     //     });
//     // }
// }

