//! board game

mod ui;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn((
            Camera {
                order: 2,
                ..default()
            },
            Camera2d,
        ));
    });

    app.add_plugins(DefaultPlugins);
    // app.add_plugins(board::BoardPlugin);
    app.add_plugins(ui::UiPlugin);

    #[cfg(not(target_family = "wasm"))]
    {
        app.add_systems(Update, keyboard_shortcuts);
    }

    app.run();
}

#[cfg(not(target_family = "wasm"))]
fn keyboard_shortcuts(mut writer: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        writer.write(AppExit::Success);
    }
    if keyboard.just_pressed(KeyCode::Space) {
        warn!("reseed");
    }
}
