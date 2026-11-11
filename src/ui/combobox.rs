use bevy::prelude::*;

use bevy::ui::{Pressed, RelativeCursorPosition};
use bevy::ui_widgets::Button;

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
            BorderColor::all(COLOR_UI_FG),
            BackgroundColor(COLOR_UI_BG.into()),
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
        (&mut UiCombobox, &Children, &RelativeCursorPosition),
        (Added<Pressed>, With<Button>),
    >,
    mut texts: Query<&mut Text>,
) {
    for (mut data, children, relative_cursor) in buttons.iter_mut() {
        let delta = match relative_cursor.normalized {
            None => 1,
            Some(pos) => {
                if pos.x < 0.0 {
                    data.names.len() - 1
                } else {
                    1
                }

                #[cfg(test)]
                mod tests {
                    use super::*;

                    #[test]
                    fn pressing_either_side_changes_the_selection() {
                        let mut app = App::new();
                        let first = app.world_mut().spawn(Text::new("<")).id();
                        let label = app.world_mut().spawn(Text::new("one")).id();
                        let last = app.world_mut().spawn(Text::new(">")).id();
                        let button = app
                            .world_mut()
                            .spawn((
                                UiCombobox {
                                    names: vec!["one".into(), "two".into(), "three".into()],
                                    index: 0,
                                },
                                Button,
                                RelativeCursorPosition {
                                    normalized: Some(Vec2::new(-0.25, 0.0)),
                                    ..default()
                                },
                                Pressed,
                            ))
                            .add_children(&[first, label, last])
                            .id();
                        app.add_systems(Update, update);
                        app.update();
                        assert_eq!(app.world().get::<UiCombobox>(button).unwrap().index, 2);
                        assert_eq!(app.world().get::<Text>(label).unwrap().0, "three");

                        app.world_mut().entity_mut(button).remove::<Pressed>();
                        app.update();
                        app.world_mut()
                            .get_mut::<RelativeCursorPosition>(button)
                            .unwrap()
                            .normalized = Some(Vec2::new(0.25, 0.0));
                        app.world_mut().entity_mut(button).insert(Pressed);
                        app.update();
                        assert_eq!(app.world().get::<UiCombobox>(button).unwrap().index, 0);
                        assert_eq!(app.world().get::<Text>(label).unwrap().0, "one");
                    }
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
