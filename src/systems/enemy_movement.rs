use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::components::{
    enemy::Enemy,
    player::Player,
    audio::SoundEffect,
};

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction * enemy.speed * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (transform, mut enemy) in enemy_query.iter_mut() {
        for window in &window_query {
            let half_enemy_size = enemy.size / 2.;

            let x_min = 0.0 + half_enemy_size;
            let x_max = window.width() - half_enemy_size;
            let y_min = 0.0 + half_enemy_size;
            let y_max = window.height() - half_enemy_size;

            if transform.translation.x < x_min || transform.translation.x > x_max {
                enemy.direction.x *= -1.;
            }

            if transform.translation.y < y_min || transform.translation.y > y_max {
                enemy.direction.y *= -1.;
            }
        }
    }
}

pub fn collisions_sound_effect_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_query: Query<(&Transform, &Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
    let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

    for (transform, enemy) in enemy_query.iter_mut() {
        let mut is_direction_changed = false;

        for window in &window_query {
            let half_enemy_size = enemy.size / 2.;

            let x_min = 0.0 + half_enemy_size;
            let x_max = window.width() - half_enemy_size;
            let y_min = 0.0 + half_enemy_size;
            let y_max = window.height() - half_enemy_size;

            if transform.translation.x < x_min || transform.translation.x > x_max {
                is_direction_changed = true;
            }

            if transform.translation.y < y_min || transform.translation.y > y_max {
                is_direction_changed = true;
            }

            if is_direction_changed {
                let sound_effect = if random::<f32>() > 0.5 {
                    sound_effect_1.clone()
                } else {
                    sound_effect_2.clone()
                };
                
                commands.spawn((
                    AudioBundle {
                        source: sound_effect.into(),
                        settings: PlaybackSettings { 
                            mode: bevy::audio::PlaybackMode::Despawn,
                            ..default()
                        },
                        ..default()
                    },
                    SoundEffect,
                ));
            }
        }
    }
}

pub fn enemy_hit_player(
    enemy_query: Query<(&Transform, &Enemy)>,
    mut player_query: Query<(&Transform, &mut Player)>,
) {
    for (player_transform, mut player) in player_query.iter_mut() {
        for (enemy_transform, enemy) in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);

            let player_radius = player.size / 2.;
            let enemy_radius = enemy.size / 2.;

            if distance < player_radius + enemy_radius {
                player.is_dead = true;
            }
        }
    }
}