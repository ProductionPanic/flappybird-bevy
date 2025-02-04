mod base;
mod bg;
mod bird;
mod pipes;
mod score;
mod systems;

use crate::game::bg::BackgroundPlugin;
use crate::game::bird::BirdPlugin;
use crate::game::pipes::PipesPlugin;
use crate::game::score::{Score, ScorePlugin};
use crate::game::systems::setup;
use crate::AppState;
use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy::prelude::{default, Resource};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .init_resource::<GameOver>()
            .add_plugins(ScorePlugin)
            .add_plugins(BirdPlugin)
            .add_plugins(PipesPlugin)
            .add_plugins(BackgroundPlugin)
            .add_plugins(base::BasePlugin)
            .add_systems(Startup, setup)
            .add_systems(OnEnter(SimulationState::GameOver), show_game_over)
            .add_systems(Update, (update_game_over).run_if(is_game_over))
            .add_systems(OnEnter(SimulationState::Running), start_sound)
            .add_systems(OnExit(AppState::Game), cleanup_game);
    }
}

#[derive(Component)]
pub struct GameItem {}

#[derive(States, Default, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
    GameOver,
}

#[derive(Resource)]
pub struct GameOver(pub bool);

impl Default for GameOver {
    fn default() -> GameOver {
        GameOver(false)
    }
}

pub fn show_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/gameover.png"),
            ..default()
        },
        Transform::from_xyz(0., 0., 1.),
        GameItem {},
    ));

    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/hit.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    ));

    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/die.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    ));
}

pub fn start_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/swoosh.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    ));
}

pub fn cleanup_game(
    mut commands: Commands,
    query: Query<Entity, With<GameItem>>,
    mut score: ResMut<Score>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    score.0 = 0;
}

pub fn update_game_over(mouse_input: Res<ButtonInput<MouseButton>>, mut commands: Commands) {
    if mouse_input.just_pressed(MouseButton::Left) {
        commands.set_state(AppState::Main);
    }
}

pub fn is_game_over(
    app_state: Res<State<AppState>>,
    simulation_state: Res<State<SimulationState>>,
) -> bool {
    app_state.eq(&AppState::Game) && simulation_state.eq(&SimulationState::GameOver)
}
