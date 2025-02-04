use crate::game::systems::game_is_running;
use crate::{AppState, SCREEN_HEIGHT};
use bevy::prelude::*;

pub const CHAR_WIDTH: f32 = 24.;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(OnExit(AppState::Game), cleanup_score)
            .add_systems(Update, update_score.run_if(game_is_running));
    }
}

#[derive(Resource)]
pub struct Score(pub u32);

impl Default for Score {
    fn default() -> Score {
        Score(0)
    }
}

#[derive(Component)]
pub struct ScoreText;

// clear all score text entities when the game is exited
pub fn cleanup_score(mut commands: Commands, query: Query<Entity, With<ScoreText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_score(
    score: Res<Score>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<ScoreText>>,
) {
    if score.is_changed() {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        let score = score.0;
        let score_str = score.to_string();
        let amount_of_chars = score_str.len() - 1;

        let mut x = -CHAR_WIDTH * amount_of_chars as f32 / 2.;

        for c in score_str.chars() {
            let digit = c.to_digit(10).unwrap();
            let sprite = match digit {
                0 => "sprites/0.png",
                1 => "sprites/1.png",
                2 => "sprites/2.png",
                3 => "sprites/3.png",
                4 => "sprites/4.png",
                5 => "sprites/5.png",
                6 => "sprites/6.png",
                7 => "sprites/7.png",
                8 => "sprites/8.png",
                9 => "sprites/9.png",
                _ => panic!("Invalid digit"),
            };
            commands.spawn((
                Sprite {
                    image: asset_server.load(sprite),
                    ..default()
                },
                Transform::from_xyz(x, SCREEN_HEIGHT / 2. - 36.0, 1.),
                ScoreText {},
            ));
            x += CHAR_WIDTH;
        }
    }
}
