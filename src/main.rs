mod input;

use crate::input::move_action::Action;
use bevy::prelude::*;
use input::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Action>::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, move_action::init_move)
        .add_systems(
            Update,
            (
                move_action::move_camera,
                draw_cursor,
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
) {
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

    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    // Define the golden angle for hue rotation.
    const GOLDEN_ANGLE: f32 = 137.507_77;
    let mut hsla = Hsla::hsl(0.0, 1.0, 0.5);
    const GRID_SIZE: i32 = 11; // 11×11 cubes for example
    let half = GRID_SIZE / 2;
    for x in -half..=half {
        for z in -half..=half {
            // The cube's center is shifted upward by 0.5 so that its bottom sits on y = 0.
            commands.spawn((
                Mesh3d(cube.clone()),
                MeshMaterial3d(materials.add(Color::from(hsla))),
                Transform::from_translation(Vec3::new(x as f32, 0.5, z as f32)),
            ));
            hsla = hsla.rotate_hue(GOLDEN_ANGLE);
        }
    }

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_child((Text::new("+"), TextColor(Color::srgba(0.9, 0.9, 0.9, 0.8))));
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    windows: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    let cursor_position = Vec2::new(windows.width() / 2.0, windows.height() / 2.0);

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        Isometry3d::new(
            point + ground.up() * 0.01,
            Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
        ),
        0.2,
        Color::WHITE,
    );
}
