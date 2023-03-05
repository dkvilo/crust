use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use std::f32::consts::PI;

use crate::GameState;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(rotater_system)
            .add_system(setup_scene_once_loaded)
            .add_system(keyboard_animation_control)
            .add_system(animation_player_update);
    }
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Component)]
struct Rotator;

fn rotater_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Rotator>>,
    game_state: ResMut<GameState>,
) {
    for mut transform in &mut query {
        if game_state.is_simulation_running {
            transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
        }
    }
}

fn animation_player_update(
    mut animation_player: Query<&mut AnimationPlayer>,
    game_state: Res<GameState>,
) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        player.set_speed(game_state.speed);
        if game_state.is_simulation_running {
            player.resume();
        } else {
            player.pause();
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Animations(vec![
        asset_server.load("models/animated/Fox.glb#Animation2"),
        asset_server.load("models/animated/Fox.glb#Animation1"),
        asset_server.load("models/animated/Fox.glb#Animation0"),
    ]));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(100.0, 100.0, 250.0)
            .looking_at(Vec3::new(0.0, 50.0, 0.0), Vec3::Y),
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.3, 0.5, 0.3)),
            ..Default::default()
        },
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/animated/Fox.glb#Scene0"),
            ..default()
        },
        Rotator,
    ));
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
    game_state: Res<GameState>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.set_speed(game_state.speed);
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_player: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
    mut game_state: ResMut<GameState>,
) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Up) {
            let speed = player.speed();
            game_state.speed = speed * 1.2;
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            let speed = player.speed();
            game_state.speed = speed * 0.8;
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            let elapsed = player.elapsed();
            game_state.elapsed = elapsed - 0.1;
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            let elapsed = player.elapsed();
            game_state.elapsed = elapsed + 0.1;
        }

        if keyboard_input.just_pressed(KeyCode::Return) {
            *current_animation = (*current_animation + 1) % animations.0.len();
            player
                .play(animations.0[*current_animation].clone_weak())
                .repeat();
        }
    }
}
