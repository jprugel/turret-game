pub mod builder;
mod game;
mod map;
mod menu;
mod ui;
use crate::builder::BuilderExt;
use crate::game::*;
use crate::map::*;
use crate::menu::*;
use crate::ui::*;
use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;
use bevy_obj::ObjPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        MenuPlugin,
        GamePlugin,
        ObjPlugin,
        MeshPickingPlugin,
    ))
    .init_state::<GameState>()
    .add_systems(Startup, setup)
    .add_systems(Startup, (setup_canvas, init_ui).chain())
    .add_systems(Startup, generate_map)
    .add_systems(Update, generator_system)
    .add_systems(Update, player_resources_ui)
    .add_systems(Update, spawn_storage_full_bubble)
    .add_systems(Startup, setup_player_resources);

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
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    game_state.set(GameState::Menu);
}

#[derive(Component)]
pub struct Canvas;

fn setup_canvas(mut commands: Commands) {
    info!("Setting up canvas");
    let canvas: Node = Node::builder()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .build();

    commands.spawn((canvas, Canvas));
}
