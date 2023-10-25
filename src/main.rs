use bevy::prelude::*;

pub mod setup;
pub mod action;

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

const WINDOW_TITLE: &str = "Bevy Ball Game";
const WINDOW_WIDTH: (f32, f32) = (640., 480.);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_TITLE.into(),
                        resolution: WINDOW_WIDTH.into(),
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