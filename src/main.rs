mod game;
mod menu;

use crate::game::GamePlugin;
use crate::menu::MenuPlugin;
use bevy::image::ImageSamplerDescriptor;
use bevy::prelude::*;
use bevy::window::WindowResolution;

pub const SCREEN_WIDTH: f32 = 288.;
pub const SCREEN_HEIGHT: f32 = 512.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                        resizable: false,
                        title: "Flappy Bird".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::nearest(),
                }),
        )
        .init_state::<AppState>()
        .add_plugins((MenuPlugin, GamePlugin))
        .run();
}

#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Main,
    Game,
}
