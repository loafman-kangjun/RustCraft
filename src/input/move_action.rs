use crate::input::mouse_tracking::CameraController;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

pub fn move_camera(mut query: Query<(&ActionState<Action>, &mut Transform), With<Camera3d>>) {
    for (action_state, mut transform) in &mut query {
        let mut direction = Vec3::ZERO;
        if action_state.pressed(&Action::Forward) {
            direction += *transform.forward();
        }
        if action_state.pressed(&Action::Backward) {
            direction -= *transform.forward();
        }
        if action_state.pressed(&Action::Right) {
            direction += *transform.right();
        }
        if action_state.pressed(&Action::Left) {
            direction -= *transform.right();
        }
        if action_state.pressed(&Action::Up) {
            direction += *transform.up();
        }
        // 新增：向下
        if action_state.pressed(&Action::Down) {
            direction -= *transform.up();
        }
        if direction != Vec3::ZERO {
            // 使用 delta_seconds_f32() 获取 f32 类型的增量时间
            transform.translation += direction.normalize() * 0.1;
        }
    }
}

pub fn init_move(mut commands: Commands, mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut input_map = InputMap::default();
    input_map.insert(Action::Forward, KeyCode::KeyW);
    input_map.insert(Action::Backward, KeyCode::KeyS);
    input_map.insert(Action::Left, KeyCode::KeyA);
    input_map.insert(Action::Right, KeyCode::KeyD);
    input_map.insert(Action::Up, KeyCode::KeyE);
    input_map.insert(Action::Down, KeyCode::KeyQ);

    let cursor_options = CursorOptions {
        visible: false,
        grab_mode: CursorGrabMode::Locked,
        hit_test: true,
    };

    let mut window = windows.single_mut();
    window.cursor_options = cursor_options;

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            hdr: true,
            ..default()
        },
        Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        InputManagerBundle::<Action> {
            input_map,
            ..default()
        },
        CameraController::default(),
    ));
}
