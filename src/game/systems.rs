use crate::game::{Score, SimulationState};
use crate::AppState;
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
}

pub fn update_score(state: Res<Score>) {
    if state.is_changed() {
        println!("Score: {}", state.0);
    }
}

pub fn game_is_running(
    app_state: Res<State<AppState>>,
    simulation_state: Res<State<SimulationState>>,
) -> bool {
    let is_in_game = app_state.eq(&AppState::Game);
    let is_running = simulation_state.eq(&SimulationState::Running);

    is_in_game && is_running
}
