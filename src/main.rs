//! spider ftw

mod background;
mod global_state;
mod material;
mod simu;
mod spider;
mod ui;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 2048 });

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
    app.add_plugins(simu::SimuPlugin);
    app.add_plugins(spider::SpiderPlugin);
    app.add_plugins(ui::UiPlugin);

    /*
    #[cfg(feature = "bevy_dev_tools")]
    {
        // fps overlay
        use bevy::color::palettes::basic::YELLOW;
        use bevy::dev_tools::fps_overlay::FpsOverlayConfig;
        use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_color: Color::from(YELLOW),
                ..default()
            },
        });
    }

    #[cfg(feature = "bevy_dev_tools")]
    {
        // wireframe toggle
        use bevy::color::palettes::basic::WHITE;
        use bevy::pbr::wireframe::WireframeConfig;
        use bevy::pbr::wireframe::WireframePlugin;
        app.insert_resource(WireframeConfig {
            global: false,
            default_color: WHITE.into(),
        });
        app.add_plugins(WireframePlugin);
        app.add_systems(
            Update,
            |mut wireframe_config: ResMut<WireframeConfig>, keyboard: Res<ButtonInput<KeyCode>>| {
                if keyboard.just_pressed(KeyCode::Space) {
                    wireframe_config.global = !wireframe_config.global;
                }
            },
        );
    }
    */

    #[cfg(not(target_family = "wasm"))]
    {
        app.add_systems(Update, keyboard_shortcuts);
    }

    app.run();
}

#[cfg(not(target_family = "wasm"))]
fn keyboard_shortcuts(
    mut writer: EventWriter<AppExit>,
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
    if keyboard.just_pressed(KeyCode::Space) {
        warn!("reseed");
    }
}
