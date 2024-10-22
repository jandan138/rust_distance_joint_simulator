use bevy::prelude::*;
use bevy::window::WindowPlugin;

mod cuboid;
mod distance_joint;
mod physics;
mod rendering;

use crate::cuboid::{Cuboid, CuboidBundle};
use crate::distance_joint::DistanceJoint;
use crate::physics::{physics_step_system, PhysicsBody};
use crate::rendering::{setup_rendering_environment, update_rendering};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Cube Rendering".to_string(),
                resolution: (800.0, 600.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup_rendering_environment)
        .add_system(physics_step_system)
        .add_system(update_rendering)
        .add_system(maintain_distance_joints)
        .run();
}

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
