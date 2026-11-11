use bevy::prelude::*;

use bevy::ui::{Pressed, RelativeCursorPosition};
use bevy::ui_widgets::Button;

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
            BorderColor::all(COLOR_UI_FG),
            BackgroundColor(COLOR_UI_BG_DISABLED.into()),
        ));
        container.with_child((Text::new(label), TextColor(COLOR_UI_FG.into())));
        ret = Some(container.id());
    });
    return ret.unwrap();
}

pub fn update(
    mut checkboxes: Query<(&mut UiCheckbox, &mut BackgroundColor), (Added<Pressed>, With<Button>)>,
) {
    for (mut data, mut bg_color) in checkboxes.iter_mut() {
        data.checked ^= true;
        *bg_color = if data.checked {
            COLOR_UI_BG.into()
        } else {
            COLOR_UI_BG_DISABLED.into()
        };
        info!("***** checkbox {} {}", data.label, data.checked);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn checkbox_toggles_once_per_press() {
            let mut app = App::new();
            let button = app
                .world_mut()
                .spawn((
                    UiCheckbox {
                        label: "test".into(),
                        checked: false,
                    },
                    Button,
                    BackgroundColor(COLOR_UI_BG_DISABLED.into()),
                    Pressed,
                ))
                .id();
            app.add_systems(Update, update);
            app.update();
            assert!(app.world().get::<UiCheckbox>(button).unwrap().checked);
            app.update();
            assert!(app.world().get::<UiCheckbox>(button).unwrap().checked);
            app.world_mut().entity_mut(button).remove::<Pressed>();
            app.update();
            app.world_mut().entity_mut(button).insert(Pressed);
            app.update();
            assert!(!app.world().get::<UiCheckbox>(button).unwrap().checked);
        }
    }
}
