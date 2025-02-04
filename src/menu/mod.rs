use crate::game::SimulationState;
use crate::AppState;
use bevy::prelude::*;

pub struct MenuPlugin;

#[derive(Component)]
pub struct Menu {}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Main), setup_menu)
            .add_systems(OnExit(AppState::Main), cleanup_menu)
            .add_systems(Update, menu_update.run_if(in_menu));
    }
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/message.png"),
            ..default()
        },
        Menu {},
    ));
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn menu_update(buttons: Res<ButtonInput<MouseButton>>, mut commands: Commands) {
    if buttons.just_pressed(MouseButton::Left) {
        commands.set_state(AppState::Game);
        commands.set_state(SimulationState::Running);
    }
}

pub fn in_menu(app_state: Res<State<AppState>>) -> bool {
    app_state.eq(&AppState::Main)
}
