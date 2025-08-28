use bevy::prelude::*;
use mlua::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, test_lua_system)
            .add_systems(Startup, load_assets);
    }
}

fn test_lua_system() {
    let lua = Lua::new();
    lua.load("print('Hello, bevy! from Lua')").exec().unwrap();
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    commands.spawn((
        Mesh3d(asset_server.load("house.obj")),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0).with_scale(Vec3::new(0.25, 0.25, 0.25)),
    ));
}
