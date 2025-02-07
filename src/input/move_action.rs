use bevy::prelude::*;
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
