use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle; // 导入正确的2D渲染包
use bevy::window::WindowPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1))) // 设置背景颜色为深色
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Circle Drawing".to_string(),
                resolution: (800.0, 600.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // 创建2D摄像机
    commands.spawn(Camera2dBundle::default());

    // 创建一个白色的圆形
    let circle_mesh = meshes.add(Mesh::from(shape::Circle {
        radius: 100.0, // 圆的半径
        ..Default::default()
    }));
    
    let white_material = materials.add(ColorMaterial::from(Color::WHITE));

    // 生成圆形实体
    commands.spawn(MaterialMesh2dBundle {
        mesh: circle_mesh.into(),
        material: white_material,
        ..Default::default()
    });
}
