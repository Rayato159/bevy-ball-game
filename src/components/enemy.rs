use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub size: f32,
    pub speed: f32,
    pub direction: Vec2,
}