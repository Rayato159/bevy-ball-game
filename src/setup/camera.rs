use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<(&Window, With<PrimaryWindow>)>,
) {
    for (window, _) in window_query.iter() {
        commands.spawn(Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        });
    }
}