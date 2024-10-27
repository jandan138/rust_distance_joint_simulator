// src/main.rs

use bevy::prelude::*;
use bevy::window::WindowPlugin;

mod cuboid;
mod distance_joint;
mod physics;
mod rendering;
mod simulation;

use crate::cuboid::{Cuboid, CuboidBundle};
use crate::distance_joint::{DistanceJoint, maintain_distance_joints};
use crate::physics::{physics_step_system, PhysicsBody};
use crate::simulation::{simulation_step_system};
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
        .add_system(simulation_step_system)
        //.add_system(maintain_distance_joints.before(physics_step_system)) // 确保在物理系统之前执行
        .add_system(update_rendering)
        .run();

    println!("维持距离约束系统已添加并预计将执行");
}
