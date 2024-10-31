// src/rendering.rs
use bevy::prelude::*;
use crate::cuboid::{Cuboid, CuboidBundle};
use crate::physics::PhysicsBody;
use crate::DistanceJoint;
use crate::rope_constraint::{RopeConstraint, RopeConstraints, maintain_rope_constraints};
use crate::entity_creators::create_cuboid_entity;

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
        transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });


    // 创建红色立方体实体
    let red_entity = create_cuboid_entity(
        &mut commands,
        &mut materials,
        &mut meshes,
        Color::rgb(1.0, 0.0, 0.0),
        2.0,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, 0.0, 0.0),
        true,
    );


    // 创建黄色立方体实体
    let yellow_entity = create_cuboid_entity(
        &mut commands,
        &mut materials,
        &mut meshes,
        Color::rgb(1.0, 1.0, 0.0),
        2.0,
        Vec3::new(5.0, 0.0, 0.0),
        Vec3::new(-0.0, 0.0, 0.0),
        false,
    );

    //Add DistanceJoint between red and yellow cuboids
    commands.entity(red_entity).insert(DistanceJoint {
    body_a: red_entity,
    body_b: yellow_entity,
    min_distance: 2.0, // Minimum distance between cubes
    max_distance: 5.0, // Maximum distance between cubes
    });

    //加这个绳子的约束系统
    commands.insert_resource(RopeConstraints::default());
    // 创建 RopeConstraint 并添加到全局资源中
    let rope_constraint = RopeConstraint {
        body_a: red_entity,
        body_b: yellow_entity,
        max_distance: 5.0,
    };
    // 获取并修改 RopeConstraints 资源来添加新的绳子约束
    commands.insert_resource(RopeConstraints {
        constraints: vec![rope_constraint] // 这里假设 `rope_constraint` 已经创建好了
    });
    
    
}



// Update rendering based on physics and Cuboid components
pub fn update_rendering(
    mut query: Query<(&mut Transform, &mut PhysicsBody, &Cuboid, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut transform, physics, cuboid, mut material_handle) in query.iter_mut() {
        // Update position
        //transform.translation += physics.velocity; // Update position based on velocity

        // Optionally adjust mesh size or material properties based on Cuboid properties
        let material = materials.get_mut(&material_handle).unwrap();
        //material.base_color = Color::rgb(0.5, 0.5, 1.0);  // Example: change color based on dynamics
    }
}
