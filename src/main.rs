use bevy::prelude::*;

mod dev;
use dev::DebugPlugin;

mod game_ui;
use game_ui::GameUIPlugin;

mod gameplay;
use gameplay::GameplayPlugin;

#[derive(Resource, Default)]
struct GameState {
    is_simulation_running: bool,
    speed: f32,
    elapsed: f32,
}

fn main() {
    App::new()
        .insert_resource(GameState {
            is_simulation_running: false,
            speed: 1.0,
            elapsed: 0.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(GameUIPlugin)
        .add_plugin(GameplayPlugin)
        .run();
}
