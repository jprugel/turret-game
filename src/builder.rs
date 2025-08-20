use bevy::prelude::*;
use bon::*;

#[derive(Builder)]
pub struct Node {
    width: Val,
    height: Val,
    margin: Option<UiRect>,
    justify_content: Option<JustifyContent>,
    align_items: Option<AlignItems>,
}

impl From<Node> for bevy::prelude::Node {
    fn from(node: Node) -> Self {
        bevy::prelude::Node {
            width: node.width,
            height: node.height,
            margin: node.margin.unwrap_or_default(),
            justify_content: node.justify_content.unwrap_or_default(),
            align_items: node.align_items.unwrap_or_default(),
            ..default()
        }
    }
}
