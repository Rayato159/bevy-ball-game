use bevy::{prelude::*, window::PrimaryWindow};

use crate::setup::player::Player;

const PLAYER_SPEED: f32 = 500.;
pub const PLAYER_SIZE: f32 = 64.;

pub fn player_movement(
    mut player_query: Query<(&mut Transform, With<Player>)>,
    keybaord_input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    for (mut transform, _) in &mut player_query {
        if keybaord_input.pressed(KeyCode::W) {
            transform.translation.y += PLAYER_SPEED * time.delta_seconds();
        }
        if keybaord_input.pressed(KeyCode::S) {
            transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
        }
        if keybaord_input.pressed(KeyCode::A) {
            transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
        }
        if keybaord_input.pressed(KeyCode::D) {
            transform.translation.x += PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn confine_player_movement_x(
    mut player_query: Query<(&mut Transform, With<Player>)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    for (mut transform, _) in &mut player_query {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.;

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

pub fn confine_player_movement_y(
    mut player_query: Query<(&mut Transform, With<Player>)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    for (mut transform, _) in &mut player_query {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.;

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