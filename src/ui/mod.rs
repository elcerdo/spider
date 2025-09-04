mod colors;
mod combobox;
mod game_done_screen;
mod track_selection_menu;

use bevy::prelude::*;

pub use game_done_screen::GameDoneScreenPlugin;
pub use track_selection_menu::TrackSelectionMenuPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, populate_ui);
        app.add_systems(
            Update,
            (combobox::update_comboboxes/* , update_ui_01, update_ui_02, animate_ui_00*/).chain(),
        );
    }
}

fn populate_ui(mut commands: Commands, _meshes: ResMut<Assets<Mesh>>) {
    let mut ui_frame = commands.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(5.0),
        top: Val::Px(5.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexEnd,
        ..default()
    });

    combobox::make_combobox(&mut ui_frame, vec!["aa", "bb", "cc"]);
    combobox::make_combobox(&mut ui_frame, vec!["x", "yy", "zzz", "wwww"]);
}
