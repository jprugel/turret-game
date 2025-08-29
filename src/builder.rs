use bevy::prelude::*;

impl BuilderExt for Node {}

pub trait BuilderExt {
    fn builder() -> NodeBuilder {
        NodeBuilder::default()
    }
}

#[derive(Default)]
pub struct NodeBuilder {
    width: Val,
    height: Val,
    right: Option<Val>,
    left: Option<Val>,
    top: Option<Val>,
    bottom: Option<Val>,
    margin: Option<UiRect>,
    justify_content: Option<JustifyContent>,
    align_items: Option<AlignItems>,
}

impl NodeBuilder {
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }

    pub fn left(mut self, left: Val) -> Self {
        self.left = Some(left);
        self
    }

    pub fn right(mut self, right: Val) -> Self {
        self.right = Some(right);
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = Some(justify_content);
        self
    }

    pub fn align_items(mut self, align_items: AlignItems) -> Self {
        self.align_items = Some(align_items);
        self
    }

    pub fn build(self) -> Node {
        let left = self.left.unwrap_or_default();
        let right = self.right.unwrap_or_default();
        let top = self.top.unwrap_or_default();
        let bottom = self.bottom.unwrap_or_default();
        let margin = self.margin.unwrap_or_default();
        let justify_content = self.justify_content.unwrap_or_default();
        let align_items = self.align_items.unwrap_or_default();

        Node {
            width: self.width,
            height: self.height,
            left,
            right,
            top,
            bottom,
            margin,
            justify_content,
            align_items,
            ..default()
        }
    }
}
