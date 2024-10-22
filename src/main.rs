//src\main.rs

use bevy::prelude::*;
use bevy::window::WindowPlugin;

mod cuboid;
use crate::cuboid::{Cuboid, CuboidBundle};

mod distance_joint;
use crate::distance_joint::DistanceJoint;

mod physics;
use crate::physics::{PhysicsBody, physics_step_system};

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
        .add_startup_system(setup)
        .add_system(physics_step_system)
        .add_system(maintain_distance_joints)
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    let red_cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));
    let red_cube_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0),
        ..Default::default()
    });
    let red_cube_physics = PhysicsBody::new(1.0);
    
    commands.spawn(PbrBundle {
        mesh: red_cube_mesh,
        material: red_cube_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    }).insert(red_cube_physics);

    let yellow_cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));
    let yellow_cube_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 1.0, 0.0),
        ..Default::default()
    });
    let yellow_cuboid = Cuboid::new(Vec3::new(2.0, 2.0, 2.0), Vec3::new(3.0, 0.0, 0.0), 1.0);
    let yellow_cube_transform = Transform::from_xyz(5.0, 0.0, 0.0);
    let yellow_cube_physics = PhysicsBody::new(1.0);
    
    commands.spawn(CuboidBundle::new(yellow_cuboid, yellow_cube_mesh, yellow_cube_material, yellow_cube_transform))
        .insert(yellow_cube_physics);
}

pub fn maintain_distance_joints(
    mut commands: Commands,
    query: Query<(Entity, &DistanceJoint, &Transform)>
) {
    for (entity, joint, transform) in query.iter() {
        let transform_a = query.get_component::<Transform>(joint.body_a).unwrap();
        let transform_b = query.get_component::<Transform>(joint.body_b).unwrap();

        let current_distance = transform_a.translation.distance(transform_b.translation);
        if current_distance < joint.min_distance || current_distance > joint.max_distance {
            commands.entity(entity).insert(Transform::default());
        }
    }
}
