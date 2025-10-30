use bevy::prelude::*;

use super::colors::*;

#[derive(Component)]
pub struct UiLabel;

pub fn make(frame: &mut EntityCommands<'_>, label: &str) -> Entity {
    let mut ret = Option::None;
    frame.with_children(|parent| {
        let node = make_wide_node();
        let mut container = parent.spawn((
            UiLabel,
            node.clone(),
            BorderColor(COLOR_UI_FG.into()),
            BackgroundColor(COLOR_UI_BG.into()),
        ));
        container.with_children(|parent| {
            let item = parent.spawn((Text::new(label), TextColor(COLOR_UI_FG.into())));
            ret = Some(item.id());
        });
    });
    return ret.unwrap();
}
