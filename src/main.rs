mod input;
mod ui;
mod gnworld;

use crate::input::move_action::Action;
use bevy::prelude::*;
use input::*;
use leafwing_input_manager::prelude::*;
use ui::state::AppState;
use gnworld::blur::gn;
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
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    gn();
    // let texture_handle: Handle<Image> = asset_server.load("blurred_noise.png");
    //
    // // 等待图片加载完成
    // let texture = asset_server.get_handle::<Image>(texture_handle.id());
    // if let Some(texture) = texture {
    //     let width = texture.size().x as u32;
    //     let height = texture.size().y as u32;
    //
    //     // 获取高度数据
    //     let height_data = texture.data.clone();
    //
    //     // 生成地形
    //     for x in 0..width {
    //         for y in 0..height {
    //             let index = (y * width + x) as usize;
    //             let height = height_data[index] as f32; // 获取高度值
    //
    //             commands.spawn(PbrBundle {
    //                 mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //                 material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //                 transform: Transform::from_xyz(x as f32, height / 2.0, y as f32), // 设置立方体位置
    //                 ..default()
    //             });
    //         }
    //     }
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
