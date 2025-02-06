use bevy::prelude::*;
use leafwing_input_manager::Actionlike;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
}

pub fn move_camera(
    mut query: Query<(&ActionState<Action>, &mut Transform), With<Camera3d>>,
) {
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
        if direction != Vec3::ZERO {
            // 使用 delta_seconds_f32() 获取 f32 类型的增量时间
            transform.translation += direction.normalize() * 0.1;
        }
    }
}

// fn draw_cursor(
//     camera_query: Single<(&Camera, &GlobalTransform)>,
//     ground: Single<&GlobalTransform, With<Ground>>,
//     windows: Single<&Window>,
//     mut gizmos: Gizmos,
// ) {
//     let (camera, camera_transform) = *camera_query;
//
//     let Some(cursor_position) = windows.cursor_position() else {
//         return;
//     };
//
//     // Calculate a ray pointing from the camera into the world based on the cursor's position.
//     let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
//         return;
//     };
//
//     // Calculate if and where the ray is hitting the ground plane.
//     let Some(distance) =
//         ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
//     else {
//         return;
//     };
//     let point = ray.get_point(distance);
//
//     // Draw a circle just above the ground plane at that position.
//     gizmos.circle(
//         Isometry3d::new(
//             point + ground.up() * 0.01,
//             Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
//         ),
//         0.2,
//         Color::WHITE,
//     );
// }