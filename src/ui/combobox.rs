use bevy::prelude::*;

use bevy::ui::RelativeCursorPosition;

use super::colors::*;

#[derive(Component, Default)]
pub struct UiCombobox {
    names: Vec<String>,
    pub index: usize,
}

pub fn make(frame: &mut EntityCommands<'_>, names: Vec<&str>) -> Entity {
    assert!(!names.is_empty());
    let mut ret = Option::None;
    frame.with_children(|parent| {
        let default_index = 0;
        let names: Vec<String> = names.into_iter().map(|aa| aa.into()).collect();
        let default_name = names[default_index].clone();
        let node = make_default_node();
        let mut container = parent.spawn((
            UiCombobox {
                names: names.clone(),
                index: default_index,
            },
            Button,
            RelativeCursorPosition::default(),
            node.clone(),
            BorderColor(COLOR_UI_FG.into()),
            BackgroundColor(COLOR_UI_BG.into()),
            Interaction::None,
        ));
        container.with_child((Text::new("<"), TextColor(COLOR_UI_FG.into())));
        container.with_child((Text::new(default_name), TextColor(COLOR_UI_FG.into())));
        container.with_child((Text::new(">"), TextColor(COLOR_UI_FG.into())));
        ret = Some(container.id());
    });
    return ret.unwrap();
}

pub fn update(
    mut buttons: Query<
        (
            &Interaction,
            &mut UiCombobox,
            &Children,
            &RelativeCursorPosition,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut texts: Query<&mut Text>,
) {
    for (interaction, mut data, children, relative_cursor) in buttons.iter_mut() {
        if matches!(*interaction, Interaction::Pressed) {
            let delta = match relative_cursor.normalized {
                None => 1,
                Some(pos) => {
                    if pos.x < 0.5 {
                        data.names.len() - 1
                    } else {
                        1
                    }
                }
            };
            data.index += delta;
            data.index %= data.names.len();
            let name = data.names[data.index].clone();
            info!("***** combobox {delta} {name}");
            assert!(children.len() == 3);
            let label = children[1];
            let mut text = texts.get_mut(label).unwrap();
            **text = name.clone();
        }
    }
}
