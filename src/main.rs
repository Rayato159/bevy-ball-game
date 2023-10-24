use bevy::prelude::*;

pub mod setup;
pub mod action;

use setup::{
    player::spawn_player,
    camera::spawn_camera
};

use action::player_movement::{
    player_movement,
    confine_player_movement_x,
    confine_player_movement_y
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera,
            spawn_player
        ))
        .add_systems(Update, (
            player_movement,
            confine_player_movement_x,
            confine_player_movement_y,
        ))
        .run()
}