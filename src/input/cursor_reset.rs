use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::LastCursorPosition;

pub fn reset_cursor(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut last_position: ResMut<LastCursorPosition>,
) {
    let mut window = windows.single_mut();
    let window_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    window.set_cursor_position(Option::from(window_center));
    last_position.position = Some(window_center);
}
