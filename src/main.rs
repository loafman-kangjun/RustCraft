mod input;

use crate::input::move_action::Action;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use input::*;
use leafwing_input_manager::prelude::*;
use crate::input::mouse_tracking::CameraController;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Action>::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_action::move_camera,
                mouse_tracking::update_camera_rotation,
            ),
        )
        .run();
}

#[derive(Component)]
struct Ground;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let cursor_options = CursorOptions {
        visible: true,
        grab_mode: CursorGrabMode::Locked,
        hit_test: true,
    };

    let mut window = windows.single_mut();
    window.cursor_options = cursor_options;

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Ground,
    ));

    // 创建光源
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 创建输入映射，将 WASD 分别映射到不同的动作
    let mut input_map = InputMap::default();
    input_map.insert(Action::Forward, KeyCode::KeyW);
    input_map.insert(Action::Backward, KeyCode::KeyS);
    input_map.insert(Action::Left, KeyCode::KeyA);
    input_map.insert(Action::Right, KeyCode::KeyD);
    input_map.insert(Action::Jump, KeyCode::Space);

    // 创建相机实体，并附加 InputManagerBundle
    commands.spawn((
        Camera3d::default(),
        Camera {
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
