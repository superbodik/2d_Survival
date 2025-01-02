mod game;

use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use game::player::{spawn_player, player_movement, camera_follow};
use game::map::{setup_map, update_map};
use game::debug::{setup_debug, debug_input, debug_ui};
use game::menu::{setup_menu, pause_input, pause_menu, handle_buttons, GameState};

fn pause_system(
    game_state: Res<GameState>,
    mut time: ResMut<Time>,
) {
    time.set_relative_speed(if game_state.paused { 0.0 } else { 1.0 });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, (setup_map, spawn_player, setup_debug, setup_menu))
        .add_systems(Update, (
            player_movement,
            camera_follow,
            update_map,
            debug_input,
            debug_ui,
            pause_input,
            pause_menu,
            handle_buttons,
            pause_system,
        ))
        .run();
}