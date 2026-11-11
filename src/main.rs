//! spider ftw

mod background;
mod global_state;
mod material;
#[cfg(feature = "debug_scene")]
mod simu;
mod spider;
mod ui;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(bevy::light::DirectionalLightShadowMap { size: 2048 });

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#mygame-canvas".into()),
            ..default()
        }),
        ..default()
    }));
    app.add_plugins(background::BackgroundPlugin);
    app.add_plugins(global_state::GlobalStatePlugin);
    app.add_plugins(material::CustomMaterialPlugin);
    #[cfg(feature = "debug_scene")]
    app.add_plugins(simu::SimuPlugin);
    app.add_plugins(spider::SpiderPlugin);
    app.add_plugins(ui::UiPlugin);

    #[cfg(not(target_family = "wasm"))]
    {
        app.add_systems(Update, keyboard_shortcuts);
    }

    app.run();
}

#[cfg(not(target_family = "wasm"))]
fn keyboard_shortcuts(
    mut writer: MessageWriter<AppExit>,
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<global_state::GlobalState>>,
) {
    use global_state::GlobalState;
    let can_quit = match state.get() {
        GlobalState::Ready => true,
        _ => false,
    };
    if can_quit && keyboard.just_pressed(KeyCode::Escape) {
        writer.write(AppExit::Success);
    }
    if keyboard.just_pressed(KeyCode::KeyT) {
        warn!("reseed");
    }
}
