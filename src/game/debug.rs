use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

#[derive(Debug, Resource)]
pub struct DebugState {
    pub show_console: bool,
    pub show_debug: bool,
    pub show_admin: bool,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            show_console: false,
            show_debug: false,
            show_admin: false,
        }
    }
}

#[derive(Component)]
pub struct DebugUI;

pub fn setup_debug(mut commands: Commands) {
    commands.init_resource::<DebugState>();
}

pub fn debug_input(
    keyboard: Res<Input<KeyCode>>,
    mut debug_state: ResMut<DebugState>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        debug_state.show_console = !debug_state.show_console;
    }
    if keyboard.just_pressed(KeyCode::F2) {
        debug_state.show_debug = !debug_state.show_debug;
    }
    if keyboard.just_pressed(KeyCode::F3) {
        debug_state.show_admin = !debug_state.show_admin;
    }
}

pub fn debug_ui(
    mut commands: Commands,
    debug_state: Res<DebugState>,
    diagnostics: Res<DiagnosticsStore>,
    player_query: Query<&Transform, With<crate::game::player::Player>>,
    query: Query<Entity, With<DebugUI>>,
) {
    // Удаляем старый UI если он есть
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if debug_state.show_debug {
        let fps = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
            .unwrap_or(0.0);

        let player_pos = player_query
            .get_single()
            .map(|transform| transform.translation)
            .unwrap_or_default();

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        right: Val::Px(10.0),
                        top: Val::Px(10.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                    ..default()
                },
                DebugUI,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_sections([
                    TextSection::new(
                        format!("FPS: {:.1}\n", fps),
                        TextStyle {
                            font_size: 20.0,
                            color: Color::GREEN,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        format!("Position: ({:.1}, {:.1})\n", player_pos.x, player_pos.y),
                        TextStyle {
                            font_size: 20.0,
                            color: Color::YELLOW,
                            ..default()
                        },
                    ),
                ]));
            });
    }

    if debug_state.show_console {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(10.0),
                        top: Val::Px(10.0),
                        width: Val::Px(300.0),
                        height: Val::Px(200.0),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                    ..default()
                },
                DebugUI,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Console",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
    }

    if debug_state.show_admin {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        right: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                    ..default()
                },
                DebugUI,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Admin Menu",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
    }
} 