use bevy::prelude::*;
use bevy::app::AppExit;

#[derive(Debug, Resource)]
pub struct GameState {
    pub paused: bool,
}

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct PauseOverlay;

#[derive(Component)]
enum MenuButton {
    Resume,
    Settings,
    Exit,
}

impl Default for GameState {
    fn default() -> Self {
        Self { paused: false }
    }
}

pub fn setup_menu(mut commands: Commands) {
    commands.init_resource::<GameState>();
}

pub fn pause_input(
    keyboard: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.paused = !game_state.paused;
    }
}

pub fn pause_menu(
    mut commands: Commands,
    game_state: Res<GameState>,
    menu_query: Query<Entity, Or<(With<PauseMenu>, With<PauseOverlay>)>>,
) {
    // Удаляем старое меню если оно есть
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if game_state.paused {
        // Затемнение фона
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            PauseOverlay,
        ));

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(35.0),
                        right: Val::Percent(35.0),
                        top: Val::Percent(30.0),
                        bottom: Val::Percent(30.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    background_color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
                    ..default()
                },
                PauseMenu,
            ))
            .with_children(|parent| {
                // Заголовок
                parent.spawn(TextBundle::from_section(
                    "Пауза",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));

                spawn_button(parent, "Продолжить", MenuButton::Resume);
                spawn_button(parent, "Настройки", MenuButton::Settings);
                spawn_button(parent, "Выйти", MenuButton::Exit);
            });
    }
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, button_type: MenuButton) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                ..default()
            },
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

pub fn handle_buttons(
    mut interaction_query: Query<
        (&Interaction, &MenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<GameState>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button_type, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    MenuButton::Resume => {
                        game_state.paused = false;
                    }
                    MenuButton::Settings => {
                        // TODO: Добавить открытие настроек
                        println!("Открываем настройки");
                    }
                    MenuButton::Exit => {
                        exit.send(AppExit);
                    }
                }
                *color = Color::rgb(0.5, 0.5, 0.5).into();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.3, 0.3, 0.3).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.2, 0.2, 0.2).into();
            }
        }
    }
} 