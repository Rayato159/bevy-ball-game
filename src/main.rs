use bevy::prelude::*;

pub mod entities;
pub mod systems;
pub mod configs;
pub mod components;

use entities::{
    player::spawn_player,
    enemy::spawn_enemies,
    camera::spawn_camera,
};

use systems::{
    player_movement::player_movement,
    player_movement::confine_player_movement_x,
    player_movement::confine_player_movement_y,
    enemy_movement::enemy_movement,
    enemy_movement::update_enemy_direction,
    enemy_movement::collisions_sound_effect_spawn,
};

use configs::camera::{
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
        .add_systems(Update, (
            enemy_movement,
            update_enemy_direction,
            collisions_sound_effect_spawn,
        ))
        .run()
}