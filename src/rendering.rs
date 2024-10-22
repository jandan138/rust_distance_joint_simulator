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

    // 创建红色立方体
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

    // 创建黄色立方体
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

// 更新渲染，根据物理和Cuboid组件更新Transform和其他视觉属性
pub fn update_rendering(
    mut query: Query<(&mut Transform, &mut PhysicsBody, &Cuboid, &mut Handle<StandardMaterial>, &mut Mesh)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut transform, physics, cuboid, mut material_handle, mut mesh) in query.iter_mut() {
        // 更新位置
        transform.translation += physics.velocity;  // 注意这里可能要用累加的方式来更新位置

        // 可以根据cuboid的大小调整mesh的大小
        *mesh = Mesh::from(shape::Cube { size: cuboid.size.length() });

        // 更新材料颜色，比如根据速度变化颜色
        let material = materials.get_mut(material_handle).unwrap();
        material.base_color = Color::rgb(0.5, 0.5, 1.0);  // 示例颜色
    }
}
