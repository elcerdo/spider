use bevy::prelude::Node;
use bevy::prelude::Srgba;

pub const COLOR_UI_BG: Srgba = bevy::color::palettes::css::WHITE;
pub const COLOR_UI_FG: Srgba = bevy::color::palettes::css::BLACK;
pub const COLOR_UI_BG_DISABLED: Srgba = bevy::color::palettes::css::GRAY;

// const COLOR_UI_RUNNING: Srgba = bevy::color::palettes::css::LIGHT_GREEN;

// pub const COLOR_UI_HOOVER: Srgba = bevy::color::palettes::css::LIGHT_GRAY;
// pub const COLOR_UI_PRESSED: Srgba = bevy::color::palettes::css::DARK_GRAY;

pub fn make_default_node() -> Node {
    use bevy::prelude::*;
    Node {
        border: UiRect::all(Val::Px(1.0)),
        padding: UiRect::all(Val::Px(4.0)),
        margin: UiRect::top(Val::Px(5.0)),
        width: Val::Px(150.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        ..default()
    }
}
