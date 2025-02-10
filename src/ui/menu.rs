use crate::ui::state::AppState;
use bevy::prelude::*;

#[derive(Component)]
pub struct MenuUI;

pub fn setup_menu(mut commands: Commands) {
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
                    parent.spawn(Text::new("Start Game"));
                });
        });
    commands.spawn((Camera2d::default(), MenuUI));
}

pub fn button_listener(
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
pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in query.iter() {
        info!("Delete menu entity: {:?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}
