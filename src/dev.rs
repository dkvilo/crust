use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::GameState;

pub struct DebugPlugin;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct DebugText;

const FONT_PATH: &str = "fonts/Alegreya-Regular.ttf";

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup)
            .add_system(text_update_system)
            .add_system(update_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size: 60.0,
                color: Color::GOLD,
            }),
            TextSection::from_style(TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size: 20.0,
                color: Color::WHITE,
            }),
        ]),
        FpsText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size: 100.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        DebugText,
    ));
}

fn update_system(
    mut state: ResMut<GameState>,
    mut query: Query<&mut Text, With<DebugText>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.is_simulation_running = !state.is_simulation_running;
    }

    for mut text in &mut query {
        if state.is_simulation_running {
            text.sections[0].value = format!("x{value:0.1}", value = state.speed);
        } else {
            text.sections[0].value = "Paused".to_string();
        }
    }
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.0}");
            }
        }
    }
}
