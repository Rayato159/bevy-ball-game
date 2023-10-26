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

            let rand_x = get_enemy_rand_x(window);
            let rand_y = get_enemy_rand_y(window);
    
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

pub fn get_enemy_rand_x(window: &Window) -> f32 {
    let rand_x = random::<f32>() * window.width();

    let half_enemy_size = ENEMY_SIZE / 2.;

    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    
    if rand_x < x_min {
        x_min
    } else if rand_x > x_max {
        x_max
    } else {
        rand_x
    }
}

pub fn get_enemy_rand_y(window: &Window) -> f32 {
    let rand_y = random::<f32>() * window.height();

    let half_enemy_size = ENEMY_SIZE / 2.;

    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;
    
    if rand_y < y_min {
        y_min
    } else if rand_y > y_max {
        y_max
    } else {
        rand_y
    }
}