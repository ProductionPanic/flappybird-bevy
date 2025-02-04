use crate::game::SimulationState;
use crate::AppState;
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn game_is_running(
    app_state: Res<State<AppState>>,
    simulation_state: Res<State<SimulationState>>,
) -> bool {
    let is_in_game = app_state.eq(&AppState::Game);
    let is_running = simulation_state.eq(&SimulationState::Running);

    is_in_game && is_running
}
