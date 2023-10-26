use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::player::Player;

pub fn player_movement(
    mut player_query: Query<(&mut Transform, &Player)>,
    keybaord_input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    for (mut transform, player) in &mut player_query {
        if keybaord_input.pressed(KeyCode::W) {
            transform.translation.y += player.speed * time.delta_seconds();
        }
        if keybaord_input.pressed(KeyCode::S) {
            transform.translation.y -= player.speed * time.delta_seconds();
        }
        if keybaord_input.pressed(KeyCode::A) {
            transform.translation.x -= player.speed * time.delta_seconds();
        }
        if keybaord_input.pressed(KeyCode::D) {
            transform.translation.x += player.speed * time.delta_seconds();
        }
    }
}

pub fn confine_player_movement_x(
    mut player_query: Query<(&mut Transform, &Player)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    for (mut transform, player) in &mut player_query {
        for window in &window_query {
            let half_player_size = player.size / 2.;

            let x_min = 0.0 + half_player_size;
            let x_max = window.width() - half_player_size;

            let mut translation = transform.translation;

            if translation.x < x_min {
                translation.x = x_min;
            } else if translation.x > x_max {
                translation.x = x_max;
            }

            transform.translation = translation;
        }
    }
}

pub fn confine_player_movement_y(
    mut player_query: Query<(&mut Transform, &Player)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    for (mut transform, player) in &mut player_query {
        for window in &window_query {
            let half_player_size = player.size / 2.;

            let y_min = 0.0 + half_player_size;
            let y_max = window.height() - half_player_size;

            let mut translation = transform.translation;

            if translation.y < y_min {
                translation.y = y_min;
            } else if translation.y > y_max {
                translation.y = y_max;
            }

            transform.translation = translation;
        }
    }
}