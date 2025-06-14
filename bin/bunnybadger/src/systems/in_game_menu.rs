use crate::plugins::start_screen::GameState;
use bevy::prelude::*;

/// System to switch to pause menu on ESC
pub fn handle_ingame_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::PauseMenu);
    }
}
