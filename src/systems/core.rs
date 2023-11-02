use bevy::{prelude::*, app::AppExit};

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        app_exit_events.send(bevy::app::AppExit);
    }
}