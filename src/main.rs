mod input;

use crate::input::move_action::Action;
use bevy::prelude::*;
use input::*;
use leafwing_input_manager::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum AppState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Action>::default())
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, button_system.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
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

// 用于标记菜单 UI 相关的实体，便于后续清理
#[derive(Component)]
struct MenuUI;

#[derive(Component)]
struct Ground;
/// 创建菜单界面：全屏根节点下嵌入一个按钮
fn setup_menu(mut commands: Commands) {
    // 使用 Node 组件构建全屏容器，自动插入所需的布局、透明度、Transform 等组件
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // 居中对齐
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            MenuUI,
        ))
        .with_children(|parent| {
            // 在容器内生成一个按钮（直接使用 Button 组件，不再依赖 ButtonBundle）
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
                ))
                .with_children(|parent| {
                    // 新版 Text 组件只需传入字符串，相关样式组件会自动插入
                    parent.spawn(Text::new("Start Game"));
                });
        });

    commands.spawn((Camera2d::default(), MenuUI));
}

/// 监测菜单中按钮的交互，点击后切换到 Game 状态
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.12, 0.18, 0.18));
                next_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.5, 0.18, 0.18));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into());
            }
        }
    }
}

/// 清理菜单界面：删除所有带有 MenuUI 标记的实体
fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in query.iter() {
        info!("Despawning menu entity: {:?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}

/// 创建 3D 场景：生成一个 3D 摄像机、一个点光源和一个蓝色立方体
fn setup_game(
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
