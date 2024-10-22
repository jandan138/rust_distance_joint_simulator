// src/rendering.rs
use bevy::prelude::*;
use crate::cuboid::Cuboid;
use crate::physics::PhysicsBody;

// 初始化渲染环境，包括灯光和相机
pub fn setup_rendering_environment(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // 设置环境光源，提供一些背景光
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    // 添加一个点光源
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });

    // 添加相机
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// 更新渲染，根据物理组件的位置更新Transform
pub fn update_rendering(
    mut query: Query<(&mut Transform, &PhysicsBody, &Cuboid)>
) {
    for (mut transform, physics, cuboid) in query.iter_mut() {
        transform.translation = physics.velocity; // 更新位置
        // 这里还可以根据 cuboid 的属性来调整 Mesh 的大小和材质
    }
}
