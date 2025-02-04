use crate::game::base::BASE_HEIGHT;
use crate::game::systems::game_is_running;
use crate::game::{GameItem, GameOver, SimulationState};
use crate::AppState;
use bevy::audio::PlaybackMode;
use bevy::prelude::*;

pub const BIRD_GRAVITY: f32 = -9.8;
pub const BIRD_HEIGHT: f32 = 24.;
pub const BIRD_WIDTH: f32 = 34.;
pub const BIRD_JUMP_SPEED: f32 = 4.;
pub const BIRD_X: f32 = -80.;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), (setup_bird))
            .add_systems(
                Update,
                (
                    flap,
                    update_bird_sprite,
                    update_bird,
                    confine_bird.after(update_bird),
                )
                    .run_if(game_is_running),
            );
    }
}

#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
    pub sprite_up: Handle<Image>,
    pub sprite_mid: Handle<Image>,
    pub sprite_down: Handle<Image>,
}

pub fn setup_bird(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut bird_upflap = "sprites/bluebird-upflap.png";
    let mut bird_midflap = "sprites/bluebird-midflap.png";
    let mut bird_downflap = "sprites/bluebird-downflap.png";

    let random = rand::random::<f32>();
    if random > 0.6 {
        bird_upflap = "sprites/redbird-upflap.png";
        bird_midflap = "sprites/redbird-midflap.png";
        bird_downflap = "sprites/redbird-downflap.png";
    } else if random > 0.3 {
        bird_upflap = "sprites/yellowbird-upflap.png";
        bird_midflap = "sprites/yellowbird-midflap.png";
        bird_downflap = "sprites/yellowbird-downflap.png";
    }

    commands.spawn((
        Bird {
            velocity: 0.,
            sprite_up: asset_server.load(bird_upflap),
            sprite_mid: asset_server.load(bird_midflap),
            sprite_down: asset_server.load(bird_downflap),
        },
        GameItem {},
        Sprite {
            image: asset_server.load(bird_midflap),
            ..default()
        },
        Transform::from_xyz(BIRD_X, 0., 0.),
    ));
}

pub fn update_bird(
    time: Res<Time>,
    mut query: Query<(&mut Bird, &mut Transform, &mut Sprite)>,
    asset_server: Res<AssetServer>,
) {
    for (mut bird, mut transform, mut sprite) in query.iter_mut() {
        bird.velocity += BIRD_GRAVITY * time.delta_secs();
        transform.translation.y += bird.velocity;

        // Rotate the bird based on its velocity
        let rotation_min = -90.;
        let rotation_max = 90.;
        let velocity = bird.velocity * 5.;

        transform.rotation =
            Quat::from_rotation_z(velocity.min(rotation_max).max(rotation_min).to_radians());
    }
}

pub fn flap(
    keyboard_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Bird>,
    game_over: Res<GameOver>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for mut bird in query.iter_mut() {
        if keyboard_input.just_pressed(MouseButton::Left) && !game_over.0 {
            bird.velocity = BIRD_JUMP_SPEED;
            let sound = asset_server.load("audio/wing.ogg");
            commands.spawn((
                AudioPlayer::new(sound),
                PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
            ));
        }
    }
}

pub fn confine_bird(mut query: Query<(&mut Bird, &mut Transform)>, mut commands: Commands) {
    let height = 256. - BIRD_HEIGHT / 2. - BASE_HEIGHT / 2.;

    for (mut bird, mut transform) in query.iter_mut() {
        if transform.translation.y < -height {
            transform.translation.y = -height;
            commands.set_state(SimulationState::GameOver);
        }
    }
}

pub fn update_bird_sprite(mut bird_query: Query<(&Bird, &mut Sprite)>) {
    for (bird, mut sprite) in bird_query.iter_mut() {
        let threshold = 1.;

        if bird.velocity > threshold {
            sprite.image = bird.sprite_up.clone();
        } else if bird.velocity < -threshold {
            sprite.image = bird.sprite_down.clone();
        } else {
            sprite.image = bird.sprite_mid.clone();
        }
    }
}
