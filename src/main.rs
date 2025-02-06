mod input;

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use leafwing_input_manager::prelude::*;
use input::*;
use crate::input::move_action::Action;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<move_action::Action>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (mouse_tracking::track_mouse_movement, cursor_reset::reset_cursor, move_action::move_camera))
        .run();
}

#[derive(Resource, Default)]
struct LastCursorPosition {
    position: Option<Vec2>,
}

#[derive(Resource, Default)]
struct MouseMotionDelta {
    delta: Vec2,
}

#[derive(Component)]
struct Ground;

fn setup(
        mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             mut windows:  Query<&mut Window, With<PrimaryWindow>>,
    ) {
    let cursor_options = CursorOptions {
        visible: true,
        grab_mode: CursorGrabMode::Locked,
        hit_test: true,
    };

    let mut window = windows.single_mut();
    window.cursor_options = cursor_options;

    commands.insert_resource(LastCursorPosition::default());
    commands.insert_resource(MouseMotionDelta::default());

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
        ));
}

