use bevy::prelude::*;
use bevy::ecs::system::Res;

use crate::physics::PhysicsBody;

// 根据速度修改最终位置
pub fn simulation_step_system(
    time: Res<Time>, 
    mut query: Query<(Entity, &mut Transform, &mut PhysicsBody)>
) {
    for (entity, mut transform, mut physics) in query.iter_mut() {
        let mass = physics.mass;  // 先复制质量到局部变量
        // 在应用物理计算后更新位置
        //let delta_transform = physics.velocity * time.delta_seconds() ;
        // 仅当 is_fixed 为 false 时更新位置
        if !physics.is_fixed {
            //physics.last_position = transform.translation;
            transform.translation = physics.predicted_position;
            physics.velocity = physics.velocity * 0.999;
        }
        else if (physics.is_fixed) {
            physics.velocity = Vec3::ZERO;
        }


        // 输出调试信息
        println!("simulation中更新后的Entity: {:?}, Position: {:?}, Velocity: {:?}, Mass: {}, 时间的变化量：{}, 位置的变化量:{:?}", 
        entity, transform.translation, physics.velocity, 1, time.delta_seconds(), transform.translation - physics.last_position);
    }
}