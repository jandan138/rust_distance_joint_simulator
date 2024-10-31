use bevy::prelude::*;
use crate::physics::PhysicsBody;  // Assuming PhysicsBody is in a module named `physics`

pub fn create_cuboid_entity(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    color: Color,
    size: f32,
    position: Vec3,
    velocity: Vec3,
    is_fixed: bool,
) -> Entity {
    let mesh = meshes.add(Mesh::from(shape::Cube { size }));
    let material = materials.add(StandardMaterial {
        base_color: color,
        ..Default::default()
    });
    let physics_body = PhysicsBody::new_PhysicsBody(1.0, velocity, Vec3::ZERO, position, is_fixed);

    commands.spawn(PbrBundle {
        mesh,
        material,
        transform: Transform::from_translation(position),
        ..Default::default()
    })
    .insert(physics_body)
    .id()
}
