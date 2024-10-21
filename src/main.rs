use bevy::prelude::*;
use bevy::window::WindowPlugin;

fn main() {
    App::new()
        // 设置窗口背景颜色为深色 (RGB: 0.1, 0.1, 0.1)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        // 启用默认插件，包括窗口插件和渲染插件
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // 设置窗口标题
                title: "3D Cube Rendering".to_string(),
                // 设置窗口的分辨率为 800x600
                resolution: (800.0, 600.0).into(),
                ..Default::default() // 其他窗口设置使用默认值
            }),
            ..Default::default() // 其他窗口插件设置使用默认值
        }))
        // 注册启动时的系统，`setup` 用于初始化场景
        .add_startup_system(setup)
        // 启动 Bevy 应用
        .run();
}

// `setup` 系统用于设置场景中的实体，包括摄像机、光源和立方体
    // Assets<Mesh> 是一种资产 (Asset) 存储类型，它用于存储和管理 Mesh 类型的对象。
    //     Assets：是 Bevy 提供的一种泛型资产容器，用于存储不同类型的资产，例如 Mesh、Material 等。
    //     Mesh：表示 3D 网格，定义了物体的几何形状。每个 Mesh 都包含顶点、边和面，用于构建 3D 物体的几何结构。
fn setup(
    mut commands: Commands, // `Commands` 用于生成和操作实体
    mut meshes: ResMut<Assets<Mesh>>, // `Assets` 用于管理和存储 Mesh (网格) 资源
    mut materials: ResMut<Assets<StandardMaterial>> // `Assets` 用于管理和存储材质资源
) {
    // 创建 3D 摄像机，放置在 (0.0, 5.0, 10.0)，并朝向原点 (0.0, 0.0, 0.0)
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y), 
        ..Default::default() // 其他设置保持默认
    });

    // 添加光源，放置在 (4.0, 8.0, 4.0) 的位置，强度为 3000，启用阴影
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0, // 设置光源的强度为 3000.0
            shadows_enabled: true, // 启用阴影效果
            ..Default::default() // 其他光源属性使用默认值
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0), // 设置光源的位置
        ..Default::default() // 其他变换属性使用默认值
    });

    // 创建一个立方体网格 (mesh) ，边长为 2.0
    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));
    
    // 创建一个红色 (RGB: 1.0, 0.0, 0.0) 的材质
    let cube_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0), // 红色材质
        ..Default::default() // 其他材质属性使用默认值
    });

    // 生成立方体实体，使用 PBR (物理基础渲染) 材质和网格
    commands.spawn(PbrBundle {
        mesh: cube_mesh.clone(), // 立方体的网格 (clone 确保安全使用)
        material: cube_material.clone(), // 立方体的材质
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // 立方体放置在场景原点
        ..Default::default() // 其他 PBR 设置使用默认值
    });
}
