use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::enemy::Enemy;

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in &mut enemy_query {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction * enemy.speed * time.delta_seconds();
    }
}

pub fn confine_enemy_movement_x(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    for (transform, mut enemy) in &mut enemy_query {
        for window in &window_query {
            let half_enemy_size = enemy.size / 2.;

            let x_min = 0.0 + half_enemy_size;
            let x_max = window.width() - half_enemy_size;

            if transform.translation.x < x_min || transform.translation.x > x_max {
                enemy.direction.x *= -1.;
            }
        }
    }
}

pub fn confine_enemy_movement_y(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    for (transform, mut enemy) in &mut enemy_query {
        for window in &window_query {
            let half_enemy_size = enemy.size / 2.;

            let y_min = 0.0 + half_enemy_size;
            let y_max = window.height() - half_enemy_size;

            if transform.translation.y < y_min || transform.translation.y > y_max {
                enemy.direction.y *= -1.;
            }
        }
    }
}
