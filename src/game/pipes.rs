use crate::game::base::BASE_HEIGHT;
use crate::game::bird::{Bird, BIRD_WIDTH};
use crate::game::systems::game_is_running;
use crate::game::{GameItem, GameOver, Score, SimulationState};
use crate::SCREEN_WIDTH;
use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use rand::Rng;

pub const PIPE_WIDTH: f32 = 52.;
pub const PIPE_HEIGHT: f32 = 320.;
pub const PIPE_SPACING: f32 = 100.;
pub const PIPE_SPAWN_DELAY: f32 = 3.;
pub const PIPE_SPEED: f32 = 50.;

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeSpawnTimer::default()).add_systems(
            Update,
            (
                update_pipes,
                tick_pipe_timer,
                move_pipes,
                check_past_bird.after(move_pipes),
                bird_hit_pipe,
            )
                .run_if(game_is_running),
        );
    }
}

#[derive(Component)]
pub struct Pipe {
    pub past_bird: bool,
}

#[derive(Resource)]
pub struct PipeSpawnTimer {
    pub timer: Timer,
}

impl Default for PipeSpawnTimer {
    fn default() -> PipeSpawnTimer {
        PipeSpawnTimer {
            timer: Timer::from_seconds(PIPE_SPAWN_DELAY, TimerMode::Repeating),
        }
    }
}

fn random_between<'a>(min: f32, max: f32) -> impl FnMut() -> f32 + 'a {
    move || min + (max - min) * rand::random::<f32>()
}

pub fn update_pipes(
    mut timer: ResMut<PipeSpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_over: Res<GameOver>,
) {
    if !game_over.0 {
        if timer.timer.just_finished() {
            let min_offset = -BASE_HEIGHT;
            let max_offset = BASE_HEIGHT + 40.;

            let offset = rand::thread_rng().gen_range(min_offset..max_offset);

            commands.spawn((
                Pipe { past_bird: false },
                Sprite {
                    image: asset_server.load("sprites/pipe-green.png"),
                    ..default()
                },
                GameItem {},
                Transform::from_xyz(
                    SCREEN_WIDTH / 2. + PIPE_WIDTH / 2.,
                    -PIPE_HEIGHT / 2. - PIPE_SPACING / 2. + offset,
                    -1.2,
                ),
                GlobalTransform::default(),
            ));
            commands.spawn((
                Pipe { past_bird: false },
                Sprite {
                    image: asset_server.load("sprites/pipe-green.png"),
                    flip_y: true,
                    ..default()
                },
                GameItem {},
                Transform::from_xyz(
                    SCREEN_WIDTH / 2. + PIPE_WIDTH / 2.,
                    PIPE_HEIGHT / 2. + PIPE_SPACING / 2. + offset,
                    -1.2,
                ),
                GlobalTransform::default(),
            ));
        }
    }
}

pub fn move_pipes(
    mut query: Query<(&mut Transform, Entity), With<Pipe>>,
    mut commands: Commands,
    time: Res<Time>,
    game_over: Res<GameOver>,
) {
    if !game_over.0 {
        for (mut transform, entity) in query.iter_mut() {
            transform.translation.x -= PIPE_SPEED * time.delta_secs();

            if transform.translation.x < (SCREEN_WIDTH / 2. + PIPE_WIDTH / 2.) * -1. {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn check_past_bird(
    mut query: Query<(&Transform, &mut Pipe)>,
    bird_query: Query<&Transform, With<Bird>>,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let mut has_passed_in_frame = false;

    for (pipe_transform, mut pipe) in query.iter_mut() {
        for bird_transform in bird_query.iter() {
            if pipe_transform.translation.x < bird_transform.translation.x {
                if !pipe.past_bird {
                    has_passed_in_frame = true;
                    pipe.past_bird = true;
                }
            }
        }
    }

    if has_passed_in_frame {
        score.0 += 1;

        commands.spawn((
            AudioPlayer::new(asset_server.load("audio/point.ogg")),
            PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
        ));
    }
}

pub fn bird_hit_pipe(
    mut commands: Commands,
    bird_query: Query<(&Transform), With<Bird>>,
    pipe_query: Query<(&Transform), With<Pipe>>,
) {
    for bird_transform in bird_query.iter() {
        let bird_min_x = bird_transform.translation.x - BIRD_WIDTH / 3.;
        let bird_max_x = bird_transform.translation.x + BIRD_WIDTH / 3.;
        let bird_min_y = bird_transform.translation.y - BIRD_WIDTH / 3.;
        let bird_max_y = bird_transform.translation.y + BIRD_WIDTH / 3.;

        for pipe_transform in pipe_query.iter() {
            let pipe_x = pipe_transform.translation.x;
            let pipe_y = pipe_transform.translation.y;

            let pipe_min_x = pipe_x - PIPE_WIDTH / 2.;
            let pipe_max_x = pipe_x + PIPE_WIDTH / 2.;
            let pipe_min_y = pipe_y - PIPE_HEIGHT / 2.;
            let pipe_max_y = pipe_y + PIPE_HEIGHT / 2.;

            if bird_max_x > pipe_min_x
                && bird_min_x < pipe_max_x
                && bird_max_y > pipe_min_y
                && bird_min_y < pipe_max_y
            {
                commands.set_state(SimulationState::GameOver);
            }
        }
    }
}

pub fn tick_pipe_timer(time: Res<Time>, mut timer: ResMut<PipeSpawnTimer>) {
    timer.timer.tick(time.delta());
}
