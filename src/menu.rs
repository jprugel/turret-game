use crate::GameState;
use crate::builder::Node as BNode;
use bevy::color::palettes::basic::*;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Menu),
            (setup_canvas, setup_buttons).chain(),
        )
        .add_systems(Update, button_system);
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                info!("Button pressed");
                game_state.set(GameState::Game);
                **text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                **text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                **text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
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

fn setup_buttons(mut commands: Commands, canvas: Single<Entity, With<Canvas>>) {
    info!("Setting up buttons");
    let button_node: Node = BNode::builder()
        .width(Val::Px(150.0))
        .height(Val::Px(65.0))
        .margin(UiRect::all(Val::Px(10.0)))
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .build()
        .into();

    commands.entity(*canvas).insert((
        button_node,
        Button,
        BackgroundColor(Color::WHITE),
        BorderColor(Color::BLACK),
        children![Text::new("Button")],
    ));
}
