use crate::Canvas;
use crate::builder::BuilderExt;
use bevy::prelude::*;

#[derive(Component)]
struct Map {
    width: u32,
    height: u32,
}

impl Map {
    fn new(width: u32, height: u32) -> Self {
        Map { width, height }
    }
}

#[derive(Component)]
struct Tile;

pub fn generate_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map = Map::new(100, 100);

    for x in 0..map.width {
        for y in 0..map.height {
            let x = x as f32 - map.height as f32 / 2.0;
            let y = y as f32 - map.width as f32 / 2.0;
            commands
                .spawn((
                    Tile,
                    Mesh3d(asset_server.load("untitled.obj")),
                    MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                    Transform::from_xyz(x, 0.0, y).with_scale(Vec3::new(0.25, 0.25, 0.25)),
                ))
                .observe(on_block_hover)
                .observe(on_block_down);
        }
    }
}

fn on_block_hover(
    hover: Trigger<Pointer<Over>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut tiles: Query<&mut MeshMaterial3d<StandardMaterial>, With<Tile>>,
) {
    info!("Hovered over tile");
    let white_material = materials.add(Color::WHITE);
    if let Ok(mut tile) = tiles.get_mut(hover.target()) {
        info!("Hovered over tile");
        tile.0 = white_material.clone();
    }
}

#[derive(Component)]
pub struct Farm;

#[derive(Component)]
pub struct Generator {
    rate: u32,
}

#[derive(Component)]
pub struct Storage {
    capacity: u32,
    amount: u32,
}

#[derive(Component)]
pub struct StorageFullEvent(Entity);

impl Storage {
    fn increase(&mut self, amount: u32) {
        self.amount += amount;
        self.amount = self.amount.min(self.capacity);
    }
}

pub fn spawn_storage_full_bubble(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    storage: Query<(&Storage, &Transform)>,
) {
    for (storage, transform) in &storage {
        if storage.amount == storage.capacity {
            commands.spawn((
                Sprite::from_image(asset_server.load("Sprite-0001.png")),
                Transform::from_translation(Vec3::new(
                    transform.translation.x,
                    transform.translation.y + 1.0,
                    transform.translation.z,
                )),
            ));
        }
    }
}

pub fn generator_system(mut generators: Query<(&mut Storage, &Generator)>, time: Res<Time>) {
    for (mut storage, generator) in &mut generators {
        storage.increase(generator.rate * time.delta().as_millis() as u32);
        info!("storage amount: {}", storage.amount);
    }
}

fn on_block_down(
    down: Trigger<Pointer<Released>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut tiles: Query<(&mut MeshMaterial3d<StandardMaterial>, &Transform), With<Tile>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("Clicked on tile");
    let white_material = materials.add(Color::BLACK);
    if let Ok((mut tile, transform)) = tiles.get_mut(down.target()) {
        info!("Clicked on tile");
        tile.0 = white_material.clone();
        let translation = transform.translation + Vec3::new(0.0, 1.0, 0.0);
        commands.spawn((
            Generator { rate: 1 },
            Storage {
                capacity: 100,
                amount: 0,
            },
            Farm,
            Transform::from_translation(translation).with_scale(Vec3::new(0.25, 0.25, 0.25)),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Mesh3d(asset_server.load("house.obj")),
        ));
    }
}
