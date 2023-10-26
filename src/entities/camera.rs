use bevy::{
    prelude::*, 
    window::PrimaryWindow, 
    core_pipeline::clear_color::ClearColorConfig
};

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<(&Window, With<PrimaryWindow>)>,
) {
    for (window, _) in window_query.iter() {
        commands.spawn(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0., 0., 0.)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        });
    }
}