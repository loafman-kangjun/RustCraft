use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::LastCursorPosition;
use crate::MouseMotionDelta;

pub fn track_mouse_movement(
    mut cursor_moved_reader: EventReader<CursorMoved>,
    mut last_position: ResMut<LastCursorPosition>,
    mut motion: ResMut<MouseMotionDelta>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let window_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    for event in cursor_moved_reader.read() {
        if let Some(last) = last_position.position {
            motion.delta = event.position - last;
        } else {
            motion.delta = Vec2::ZERO;
        }
        last_position.position = Some(event.position);
    }

    if last_position.position.is_none() {
        last_position.position = Some(window_center);
    }

    println!("Mouse Delta: {:?}", motion.delta);
}
