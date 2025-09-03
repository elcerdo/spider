use crate::global_state::GlobalState;

use bevy::prelude::*;

use super::consts::*;

pub struct GameDoneScreenPlugin;

impl Plugin for GameDoneScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GlobalState::GameDone), populate_screen);
        app.add_systems(OnExit(GlobalState::GameDone), depopulate_screen);
        app.add_systems(
            Update,
            to_track_selection_menu.run_if(in_state(GlobalState::GameDone)),
        );
    }
}

#[derive(Component)]
struct GameDoneScreenMarker;

fn depopulate_screen(mut commands: Commands, query: Query<Entity, With<GameDoneScreenMarker>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

fn populate_screen(mut commands: Commands) {
    commands.spawn((GameDoneScreenMarker, Camera2d));
    commands
        .spawn((
            GameDoneScreenMarker,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("DONE"),
                TextFont {
                    font_size: 25.0,
                    ..default()
                },
                TextColor(COLOR_UI_BG.into()),
            ));
            parent.spawn((
                Text::new("press escape"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(COLOR_UI_BG.into()),
            ));
        });
}

fn to_track_selection_menu(
    mut next_state: ResMut<NextState<GlobalState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GlobalState::TrackSelectionInit);
    }
}
