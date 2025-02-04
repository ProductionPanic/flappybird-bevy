use crate::game::pipes::PIPE_SPEED;
use crate::AppState;
use bevy::prelude::*;

pub const BASE_HEIGHT: f32 = 112.;
pub const BASE_WIDTH: f32 = 336.;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_base).add_systems(
            Update,
            (move_bases, confine_bases).chain().run_if(check_base_state),
        );
    }
}

#[derive(Component)]
pub struct Base {}

pub fn setup_base(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..3 {
        commands.spawn((
            Base {},
            Sprite {
                image: asset_server.load("sprites/base.png"),
                ..default()
            },
            Transform::from_xyz(BASE_WIDTH * i as f32, -256., -1.),
        ));
    }
}

pub fn confine_bases(mut query: Query<(&Base, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        if transform.translation.x <= -BASE_WIDTH {
            transform.translation.x += BASE_WIDTH * 3.;
        }
    }
}

pub fn move_bases(time: Res<Time>, mut query: Query<(&Base, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        transform.translation.x -= PIPE_SPEED * time.delta_secs();
    }
}

pub fn check_base_state(
    app_state: Res<State<AppState>>,
    simulation_state: Res<State<super::SimulationState>>,
) -> bool {
    if app_state.eq(&AppState::Main) {
        true
    } else if simulation_state.eq(&super::SimulationState::Running) {
        true
    } else {
        false
    }
}
