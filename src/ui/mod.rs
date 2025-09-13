mod checkbox;
mod colors;
mod combobox;
// mod game_done_screen;
// mod track_selection_menu;

use bevy::prelude::*;

use checkbox::UiCheckbox;

// pub use game_done_screen::GameDoneScreenPlugin;
// pub use track_selection_menu::TrackSelectionMenuPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, populate_ui);
        app.add_systems(Update, (combobox::update, checkbox::update, update).chain());
    }
}

#[derive(Resource)]
pub struct UiState {
    toggle_gizmos: Entity,
    pub display_gizmos: bool,
}

fn populate_ui(mut commands: Commands) {
    let mut ui_frame = commands.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(5.0),
        top: Val::Px(5.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexEnd,
        ..default()
    });

    combobox::make(&mut ui_frame, vec!["aa", "bb", "cc"]);
    combobox::make(&mut ui_frame, vec!["x", "yy", "zzz", "wwww"]);

    let toggle_gizmos = checkbox::make(&mut ui_frame, "gizmos");

    commands.insert_resource(UiState {
        toggle_gizmos,
        display_gizmos: false,
    });
}

fn update(mut ui_state: ResMut<UiState>, checkboxes: Query<&UiCheckbox>) {
    let foo = checkboxes.get(ui_state.toggle_gizmos).unwrap();
    ui_state.display_gizmos = foo.checked;
}
