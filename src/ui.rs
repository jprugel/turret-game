use crate::Canvas;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerResources {
    gold: u32,
    food: u32,
}

#[derive(Component)]
pub struct Ui;

pub fn init_ui(mut commands: Commands, mut canvas: Single<Entity, With<Canvas>>) {
    commands
        .entity(*canvas)
        .insert(Text::new(format!("Gold: {}, Food: {}", 0, 0)));
}

pub fn player_resources_ui(
    player_resources: Query<&PlayerResources>,
    mut player_resources_ui: Query<&mut TextSpan, With<Ui>>,
) {
    // Implement UI for player resources
    for (player_resources, mut ui) in player_resources.iter().zip(player_resources_ui.iter_mut()) {
        **ui = format!(
            "Gold: {}, Food: {}",
            player_resources.gold, player_resources.food
        );
    }
}
