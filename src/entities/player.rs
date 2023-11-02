use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::player::Player;
use crate::configs::player::{
    PLAYER_SIZE,
    PLAYER_SPEED,
};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<(&Window, With<PrimaryWindow>)>,
    asset_server: Res<AssetServer>,
) {
    let player_texture = asset_server.load("sprites/ball_blue_large.png");

    for (window, _) in window_query.iter() {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2., window.height() / 2.,  0.),
                texture: player_texture.clone(),
                ..default()
            },
            Player {
                speed: PLAYER_SPEED,
                size: PLAYER_SIZE,
                is_dead: false,
            },
        ));
    }
}