use bevy::prelude::*;

pub mod setup;
pub mod action;
pub mod config;
pub mod components;

use setup::{
    player::spawn_player,
    enemy::spawn_enemies,
    camera::spawn_camera
};

use action::player_movement::{
    player_movement,
    confine_player_movement_x,
    confine_player_movement_y
};

use config::camera::{
    WINDOW_TITLE,
    WINDOW_RESOLUTION,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_TITLE.into(),
                        resolution: WINDOW_RESOLUTION.into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build()
        )
        .add_systems(Startup, (
            spawn_camera,
            spawn_player,
            spawn_enemies,
        ))
        .add_systems(Update, (
            player_movement,
            confine_player_movement_x,
            confine_player_movement_y,
        ))
        .run()
}