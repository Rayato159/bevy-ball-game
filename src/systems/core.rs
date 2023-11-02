use bevy::{prelude::*, app::AppExit};

use crate::components::{
    player::Player,
    enemy::Enemy,
    audio::SoundEffect
};

pub fn game_over(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Player)>,
    mut enemy_query: Query<&mut Enemy>,
    asset_server: Res<AssetServer>,
) {
    let dead_sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");

    for (player_entity, player) in player_query.iter_mut() {
        for mut enemy in enemy_query.iter_mut() {
            if player.is_dead {
                commands.spawn((
                    AudioBundle {
                        source: dead_sound_effect.clone().into(),
                        settings: PlaybackSettings { 
                            mode: bevy::audio::PlaybackMode::Despawn,
                            ..default()
                        },
                        ..default()
                    },
                    SoundEffect,
                ));

                commands.entity(player_entity).despawn();
                
                enemy.speed = 0.0;
            }

        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(bevy::app::AppExit);
    }
}