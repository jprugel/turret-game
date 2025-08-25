pub mod builder;
mod game;
mod menu;

use crate::builder::Node as BNode;
use crate::game::*;
use crate::menu::*;
use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        MenuPlugin,
        GamePlugin,
    ))
    .init_state::<GameState>()
    .add_systems(Startup, setup)
    .add_systems(Startup, setup_canvas);

    load_internal_binary_asset!(
        app,
        TextFont::default().font,
        ".././assets/fonts/slkscr.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    let _ = app.run();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, States, Default)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Game,
    Pause,
}

fn setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    info!("Setting up camera.");
    commands.spawn(Camera2d);
    game_state.set(GameState::Menu);
}

#[derive(Component)]
pub struct Canvas;

fn setup_canvas(mut commands: Commands) {
    info!("Setting up canvas");
    let canvas: Node = BNode::builder()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .build()
        .into();
    commands.spawn((canvas, Canvas));
}
