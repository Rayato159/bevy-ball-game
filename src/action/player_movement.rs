use bevy::{prelude::*, window::PrimaryWindow};

use crate::setup::player::Player;

const PLAYER_SPEED: f32 = 500.;
const PLAYER_SIZE: f32 = 64.;

pub fn player_movement(
    keybaord_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        
        if keybaord_input.pressed(KeyCode::W) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keybaord_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0., 1., 0.);
        }
        if keybaord_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1., 0., 0.);
        }
        if keybaord_input.pressed(KeyCode::D) {
            direction += Vec3::new(1., 0., 0.);
        }

        if direction.length() > 0. {
            direction = direction.normalize();
        }

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement_x(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.;

        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        player_transform.translation = translation;
    }
}

pub fn confine_player_movement_y(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.;

        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        player_transform.translation = translation;

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}