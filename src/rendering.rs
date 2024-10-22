// src/rendering.rs
use bevy::prelude::*;
use crate::cuboid::{Cuboid, CuboidBundle};
use crate::physics::PhysicsBody;

// Initialize the rendering environment including lights and camera
pub fn setup_rendering_environment(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Setup ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    // Add a point light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });

    // Add a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Create a red cuboid
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

    // Create a yellow cuboid
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

// Update rendering based on physics and Cuboid components
pub fn update_rendering(
    mut query: Query<(&mut Transform, &mut PhysicsBody, &Cuboid, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut transform, physics, cuboid, mut material_handle) in query.iter_mut() {
        // Update position
        transform.translation += physics.velocity; // Update position based on velocity

        // Optionally adjust mesh size or material properties based on Cuboid properties
        let material = materials.get_mut(&material_handle).unwrap();
        material.base_color = Color::rgb(0.5, 0.5, 1.0);  // Example: change color based on dynamics
    }
}
