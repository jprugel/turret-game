use crate::Canvas;
use crate::GameState;
use crate::builder::Node as BNode;
use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy::winit::cursor::{CursorIcon, CustomCursor, CustomCursorImage};
use std::collections::VecDeque;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(OnEnter(GameState::Game), setup_deck)
            .add_systems(OnEnter(GameState::Game), setup_cursor_icon)
            .add_systems(OnEnter(GameState::Game), on_resize_system);
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
struct SpriteHandle(usize);

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
struct Card {
    name: String,
    card_type: CardType,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
enum CardType {
    Construct(Construct),
    Turret(Turret),
    Effect(Effect),
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
struct Effect {
    name: String,
}

struct Deck {
    cards: VecDeque<Card>,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
struct Construct {
    level: u32,
    health: u32,
    sprite: SpriteHandle,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
struct Turret {
    construct: Construct,
    //bullet: Bullet,
    head: SpriteHandle,
}

struct Bullet {
    damage: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
struct Ether(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
struct CoreDrill {
    level: u32,
    ether_rate: u32,
}

fn setup_deck(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
    canvas: Single<Entity, With<Canvas>>,
) {
    let core_drill = Construct {
        level: 1,
        health: 100,
        sprite: SpriteHandle(0),
    };

    let core_drill_card = Card {
        name: "Core Drill".to_string(),
        card_type: CardType::Construct(core_drill),
    };

    let image = asset_server.load("card.png");

    let image_node = ImageNode::new(image);

    let card_node: Node = BNode::builder()
        .height(Val::Px(150.0))
        .width(Val::Px(100.0))
        .justify_content(JustifyContent::Center)
        .build()
        .into();

    commands
        .entity(*canvas)
        .insert(children![(card_node, children![image_node])])
        .observe(on_card_hover)
        .observe(on_card_out)
        .observe(on_card_press);
}

fn on_resize_system(
    mut cards: Query<&mut Transform, With<Card>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        // When resolution is being changed
        info!("Window is being resized");
        for mut card in cards.iter_mut() {
            card.translation.y = -e.height / 2.0;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprite_sheet.png");
    let turret = Turret {
        construct: Construct {
            level: 1,
            health: 100,
            sprite: SpriteHandle(0),
        },
        //bullet: Bullet { damage: 10 },
        head: SpriteHandle(1),
    };
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Spawn the turret base (parent)
    let turret_base = commands
        .spawn((
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: turret.construct.sprite.0,
                },
            ),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();

    // Spawn the turret head (child)
    let turret_head = commands
        .spawn((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: turret.head.0,
                },
            ),
            Transform::from_xyz(0.0, 0.0, 1.0), // Higher z-value to render on top
        ))
        .id();

    // Set up parent-child relationship
    commands.entity(turret_base).add_children(&[turret_head]);
}

fn on_card_hover(hover: Trigger<Pointer<Over>>, mut transform: Query<&mut Transform, With<Card>>) {
    dbg!("Card Hovered");
    if let Ok(mut transform) = transform.get_mut(hover.target()) {
        transform.translation = Vec3::new(0.0, -215.0, 0.0);
        transform.scale = Vec3::new(2.0, 2.0, 1.0);
    }
}

fn on_card_out(out: Trigger<Pointer<Out>>, mut transform: Query<&mut Transform, With<Card>>) {
    dbg!("Card Out");
    if let Ok(mut transform) = transform.get_mut(out.target()) {
        transform.translation = Vec3::new(0.0, -350.0, 0.0);
        transform.scale = Vec3::new(1.0, 1.0, 1.0);
    }
}

fn on_card_press(
    press: Trigger<Pointer<Pressed>>,
    mut transform: Query<&mut Transform, With<Card>>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    window: Single<&Window>,
) {
    // I will want to store this information for later use.
    if let Ok(mut transform) = transform.get_mut(press.target()) {
        dbg!(window.cursor_position().unwrap());
        dbg!(transform.translation - window.cursor_position().unwrap().extend(0.0));
    }
}

fn setup_cursor_icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    window: Single<Entity, With<Window>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 10, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: asset_server.load("sprite_sheet.png"),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 46,
            }),
            ..default()
        })));
}
