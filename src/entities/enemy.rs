use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::random;

use crate::components::enemy::Enemy;
use crate::configs::enemy::{
    ENEMY_SIZE,
    NUMBER_OF_ENEMIES,
};

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<(&Window, With<PrimaryWindow>)>,
    asset_server: Res<AssetServer>,
) {
    for (window, _) in window_query.iter() {
        for _ in 0..NUMBER_OF_ENEMIES {
            let rand_x = random::<f32>() * window.width();
            let rand_y = random::<f32>() * window.height();
    
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(rand_x, rand_y,  0.),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    size: ENEMY_SIZE,
                },
            ));
        }
    }
}