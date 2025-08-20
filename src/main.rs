mod asset;
pub mod builder;
mod game;
mod menu;

use crate::menu::*;
use asset::json5::{Json5Asset, Json5Loader};
use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        MenuPlugin,
    ))
    .init_asset_loader::<Json5Loader>()
    .init_asset::<Json5Asset>()
    .init_state::<GameState>()
    .add_systems(Startup, setup)
    .add_systems(Update, test_asset);

    load_internal_binary_asset!(
        app,
        TextFont::default().font,
        ".././assets/fonts/slkscr.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    let _ = app.run();
}

fn test_asset(asset_server: Res<AssetServer>) {
    let json5_asset: Handle<Json5Asset> = asset_server.load("cards/ether_drill.json5");
    info!("Loaded asset: {:?}", json5_asset);
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
