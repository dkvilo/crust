use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(button_system)
            .add_system(fainbow_text_system);
    }
}

#[derive(Component)]
struct Button;

#[derive(Component)]
struct RainbowTextLabel;

const FONT_PATH: &str = "fonts/Alegreya-Regular.ttf";

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Hello, World!",
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
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        RainbowTextLabel,
    ));

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(50.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        right: Val::Px(10.0),
                        top: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..Default::default()
            },
            Button,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Button",
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn make_button_value(game_state: &GameState) -> &str {
    let mut _text: &str = "Play";
    if game_state.is_simulation_running {
        _text = "Stop";
    } else {
        _text = "Play";
    }
    _text
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                game_state.is_simulation_running = !game_state.is_simulation_running;
                *color = PRESSED_BUTTON.into();
            }

            Interaction::Hovered => {
                text.sections[0].value = make_button_value(&game_state).into();
                *color = HOVERED_BUTTON.into();
            }

            Interaction::None => {
                text.sections[0].value = make_button_value(&game_state).into();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn fainbow_text_system(time: Res<Time>, mut query: Query<&mut Text, With<RainbowTextLabel>>) {
    for mut text in &mut query {
        let seconds = time.elapsed_seconds();
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}
