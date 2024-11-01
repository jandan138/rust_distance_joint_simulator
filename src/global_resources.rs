// src/global_resources.rs
use bevy::prelude::Entity;
use bevy::ecs::system::Resource;

#[derive(Resource, Default)]
pub struct GlobalEntities {
    pub global_entities: Vec<Entity>,
}
