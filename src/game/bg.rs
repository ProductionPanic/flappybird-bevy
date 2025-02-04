use crate::game::pipes::PIPE_SPEED;
use crate::AppState;
use bevy::prelude::*;
use rand::random;

pub const BG_WIDTH: f32 = 288.;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bg)
            .add_systems(Update, (move_bg, confine_bg).chain().run_if(check_bg_state));
    }
}

#[derive(Component)]
pub struct Background {}

pub fn setup_bg(mut commands: Commands, asset_server: Res<AssetServer>) {
    let is_night = random::<bool>();
    let mut img: Handle<Image> = asset_server.load("sprites/background-day.png");
    if is_night {
        img = asset_server.load("sprites/background-night.png");
    }
    for i in 0..3 {
        commands.spawn((
            Background {},
            Sprite {
                image: img.clone(),
                ..default()
            },
            Transform::from_xyz(BG_WIDTH * i as f32, 0., -1.5),
        ));
    }
}

pub fn confine_bg(mut query: Query<(&Background, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        if transform.translation.x <= -BG_WIDTH {
            transform.translation.x += BG_WIDTH * 3.;
        }
    }
}

pub fn move_bg(time: Res<Time>, mut query: Query<(&Background, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        transform.translation.x -= (PIPE_SPEED / 2.) * time.delta_secs();
    }
}

pub fn check_bg_state(
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
