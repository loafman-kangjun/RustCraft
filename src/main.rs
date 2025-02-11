mod gnworld;
mod input;
mod ui;

use crate::input::move_action::Action;
use bevy::prelude::*;
use gnworld::blur::gn;
use image::{GenericImageView, Luma, Pixel};
use input::*;
use leafwing_input_manager::prelude::*;
use ui::state::AppState;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Action>::default())
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::Menu), ui::menu::setup_menu)
        .add_systems(
            Update,
            ui::menu::button_listener.run_if(in_state(AppState::Menu)),
        )
        .add_systems(OnExit(AppState::Menu), ui::menu::cleanup_menu)
        .add_systems(OnEnter(AppState::Game), setup_game)
        .add_systems(OnEnter(AppState::Game), move_action::init_move)
        .add_systems(
            Update,
            (
                move_action::move_camera.run_if(in_state(AppState::Game)),
                raycast::draw_cursor.run_if(in_state(AppState::Game)),
                mouse_tracking::update_camera_rotation.run_if(in_state(AppState::Game)),
                raycast::raycast_system.run_if(in_state(AppState::Game)),
            ),
        )
        .run();
}
#[derive(Component)]
struct Ground;

/// 创建 3D 场景：生成一个 3D 摄像机、一个点光源和一个蓝色立方体
fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    gn();

    let img = image::open("blurred_noise.png").unwrap();
    let (width, height) = img.dimensions();
    let img = img.grayscale();

    // 创建立方体
    for x in 0..width {
        for y in 0..height {
            let pixel: Luma<u8> = img.get_pixel(x, y).to_luma();
            let height = pixel[0] as f32 / 255.0 * 10.0; // 调整高度比例

            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
                MeshMaterial3d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
                Transform::from_xyz(x as f32, height, y as f32),
            ));
        }
    }

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

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
        Transform::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
        Transform::from_xyz(3.0, 0.0, 0.0),
    ));

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
