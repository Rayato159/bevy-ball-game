use bevy::prelude::*;

pub mod setup;

use setup::player::spawn_player;
use setup::camera::spawn_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera,
            spawn_player
        ))
        .run()
}