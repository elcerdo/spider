use bevy::prelude::*;

use bevy::ui::RelativeCursorPosition;

use super::colors::*;

#[derive(Component)]
pub struct UiCheckbox {
    label: String,
    pub checked: bool,
}

pub fn make(frame: &mut EntityCommands<'_>, label: &str) -> Entity {
    let mut ret = Option::None;
    frame.with_children(|parent| {
        let node = make_default_node();
        let mut container = parent.spawn((
            UiCheckbox {
                label: label.into(),
                checked: false,
            },
            Button,
            RelativeCursorPosition::default(),
            node.clone(),
            BorderColor(COLOR_UI_FG.into()),
            BackgroundColor(COLOR_UI_BG_DISABLED.into()),
            Interaction::None,
        ));
        container.with_child((Text::new(label), TextColor(COLOR_UI_FG.into())));
        ret = Some(container.id());
    });
    return ret.unwrap();
}

pub fn update(
    mut checkboxes: Query<
        (&Interaction, &mut UiCheckbox, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut data, mut bg_color) in checkboxes.iter_mut() {
        if matches!(*interaction, Interaction::Pressed) {
            data.checked ^= true;
            *bg_color = if data.checked {
                COLOR_UI_BG.into()
            } else {
                COLOR_UI_BG_DISABLED.into()
            };
            info!("***** checkbox {} {}", data.label, data.checked);
        }
    }
}
