use crate::Canvas;
use crate::builder::BuilderExt;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerResources {
    pub gold: u32,
    pub food: u32,
}

pub fn setup_player_resources(mut commands: Commands) {
    info!("For shits");
    commands.spawn(PlayerResources { gold: 0, food: 0 });
}

#[derive(Component)]
pub struct Ui;

pub fn init_ui(mut commands: Commands, canvas: Single<Entity, With<Canvas>>) {
    commands
        .entity(*canvas)
        .insert((Text::new(format!("Gold: {}, Food: {}", 0, 0)), Ui));
}

pub fn player_resources_ui(
    player_resources: Single<&PlayerResources>,
    mut player_resources_ui: Query<&mut Text, With<Ui>>,
) {
    // Implement UI for player resources

    for mut ui in player_resources_ui.iter_mut() {
        info!("Test");
        **ui = format!(
            "Gold: {}, Food: {}",
            player_resources.gold, player_resources.food
        );
    }
}

#[derive(Component)]
pub struct BuilderUi;

pub fn spawn_builder_ui(
    released: Trigger<Pointer<Released>>,
    canvas: Single<Entity, With<Canvas>>,
    mut commands: Commands,
) {
    let builder_node = Node::builder()
        .width(Val::Percent(80.))
        .height(Val::Percent(80.))
        .build();

    let title_bar = Node::builder()
        .width(Val::Percent(100.))
        .height(Val::Percent(10.))
        .build();

    let close_button = Node::builder()
        .width(Val::Percent(10.))
        .height(Val::Percent(100.))
        .left(Val::Percent(90.))
        .build();

    let button_node = Node::builder()
        .width(Val::Percent(50.))
        .height(Val::Percent(50.))
        .build();

    commands.entity(*canvas).insert((
        builder_node,
        children![(title_bar, children![close_button]), button_node],
    ));
}

pub fn despawn_builder_ui(mut commands: Commands, builder_ui: Query<Entity, With<BuilderUi>>) {
    for entity in builder_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
