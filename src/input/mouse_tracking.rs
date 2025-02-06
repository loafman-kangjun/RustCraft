use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Component)]
pub struct CameraController {
    pub pitch: f32,
    pub yaw: f32,
    pub sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 0.005, // 可根据需要调整
        }
    }
}

pub fn update_camera_rotation(
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera3d>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut total_delta = Vec2::ZERO;
    // 累计这一帧内所有的鼠标相对移动
    for event in mouse_motion_events.read() {
        total_delta += event.delta;
    }

    if total_delta == Vec2::ZERO {
        return;
    }

    for (mut transform, mut controller) in query.iter_mut() {
        controller.yaw -= total_delta.x * controller.sensitivity * time.delta().as_secs_f32();
        controller.pitch -= total_delta.y * controller.sensitivity * time.delta().as_secs_f32();
        controller.pitch = controller
            .pitch
            .clamp(-89f32.to_radians(), 89f32.to_radians());
        let yaw_rotation = Quat::from_axis_angle(Vec3::Y, controller.yaw);
        let pitch_rotation = Quat::from_axis_angle(Vec3::X, controller.pitch);

        transform.rotation = yaw_rotation * pitch_rotation;
    }
}
