use bevy::prelude::*;
use bevy::window::WindowPlugin;

mod cuboid; // 声明 cuboid 模块
use crate::cuboid::{Cuboid, CuboidBundle}; // 引入你定义的 Cuboid 和 CuboidBundle

mod distance_joint; // 导入distance_joint模块
use crate::distance_joint::DistanceJoint; // 导入DistanceJoint结构体


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1))) // 设置背景颜色为深色
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Cube Rendering".to_string(),
                resolution: (800.0, 600.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // 创建3D摄像机
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y), // 摄像机位置和视角
        ..Default::default()
    });

    info!("摄像机已生成");

    // 添加光源
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0, // 设置高光强度，确保物体被照亮
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0), // 光源位置
        ..Default::default()
    });

    info!("光源已生成");

    // 创建第一个红色立方体
    let red_cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 2.0 })); // 立方体大小
    let red_cube_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0), // 红色
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: red_cube_mesh.clone(),
        material: red_cube_material.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // 红色立方体放在原点
        ..Default::default()
    });

    info!("红色立方体已生成，位置: (0.0, 0.0, 0.0)");

     // 创建黄色立方体
     let yellow_cuboid = Cuboid::new(Vec3::new(2.0, 2.0, 2.0), Vec3::new(3.0, 0.0, 0.0), 1.0); // 立方体位置在 (3.0, 0.0, 0.0)
     let yellow_cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 2.0 })); // 立方体大小
     let yellow_cube_material = materials.add(StandardMaterial {
         base_color: Color::rgb(1.0, 1.0, 0.0), // 黄色
         ..Default::default()
     });
     let yellow_cube_transform = Transform::from_xyz(5.0, 0.0, 0.0); // 放在 (5.0, 0.0, 0.0)
     
     commands.spawn(CuboidBundle::new(yellow_cuboid, yellow_cube_mesh, yellow_cube_material, yellow_cube_transform));
     
 
     info!("黄色立方体已生成，位置: (5.0, 0.0, 0.0)");
}

pub fn maintain_distance_joints(
    mut commands: Commands,
    query: Query<(Entity, &DistanceJoint, &Transform)>,
) {
    for (entity, joint, transform) in query.iter() {
        let transform_a = query.get_component::<Transform>(joint.body_a).unwrap();
        let transform_b = query.get_component::<Transform>(joint.body_b).unwrap();

        // 计算当前距离
        let current_distance = transform_a.translation.distance(transform_b.translation);

        // 根据距离调整实体位置
        if current_distance < joint.min_distance || current_distance > joint.max_distance {
            commands.entity(entity).insert(Transform::default()); // 示例：重置位置
        }
    }
}