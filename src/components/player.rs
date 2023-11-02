use bevy::prelude::*;

#[derive(Component)]

pub struct Player {
    pub speed: f32,
    pub size: f32,
    pub is_dead: bool,
}