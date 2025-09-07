use bevy::color::palettes::css::*;
use bevy::prelude::*;

mod data;
mod physics;

use data::SpiderData;

const MODEL_SPIDER_PATH: &str = "models/tachikoma.glb";
// const MODEL_SPIDER_SCALE: f32 = 1.0;

pub struct SpiderPlugin;

impl Plugin for SpiderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (populate_spider).chain());
        // app.init_gizmo_group::<MyRoundGizmos>();
        app.add_systems(
            Update,
            (
                // exit_game,
                update_gizmos,
                reset_vehicle_positions,
                physics::update_vehicle_physics,
                // collision::bounce_and_resolve_checkpoints,
                // update_statuses,
                // update_boards_and_cups,
            )
                .chain(),
            // .run_if(in_state(state)),
        );
    }
}

// #[derive(Default, Reflect, GizmoConfigGroup)]
// struct MyRoundGizmos;

//////////////////////////////////////////////////////////////////////

// fn exit_game(mut next_state: ResMut<NextState<GlobalState>>, keyboard: Res<ButtonInput<KeyCode>>) {
//     if keyboard.just_pressed(KeyCode::Escape) {
//         next_state.set(GlobalState::GameDone);
//     }
// }

fn reset_vehicle_positions(
    mut vehicles: Query<&mut SpiderData>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        for mut vehicle in &mut vehicles {
            vehicle.reset();
        }
    }
}

//////////////////////////////////////////////////////////////////////

fn populate_spider(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // tracks: Res<Assets<Track>>,
    // state: Res<State<GlobalState>>,
) {
    let scene: Handle<Scene> = server.load(GltfAssetLabel::Scene(0).from_asset(MODEL_SPIDER_PATH));

    let mut scene = commands.spawn((
        SceneRoot(scene.clone()),
        SpiderData::from_position_forward(Vec2::ZERO, Vec2::X),
        Transform::from_translation(Vec3::Z * 5.0),
    ));

    scene.with_children(|parent| {
        let mut gizmo = GizmoAsset::new();
        gizmo.arrow(Vec3::ZERO, Vec3::X * 5.0, RED);
        parent.spawn(Gizmo {
            handle: gizmo_assets.add(gizmo),
            ..default()
        });
        let mut gizmo = GizmoAsset::new();
        gizmo.arrow(Vec3::ZERO, Vec3::Y * 5.0, GREEN);
        parent.spawn(Gizmo {
            handle: gizmo_assets.add(gizmo),
            ..default()
        });
        let mut gizmo = GizmoAsset::new();
        gizmo.arrow(Vec3::ZERO, Vec3::Z * 5.0, BLUE);
        parent.spawn(Gizmo {
            handle: gizmo_assets.add(gizmo),
            ..default()
        });
    });
}

fn update_gizmos(
    mut vehicles: Query<&mut SpiderData>,
    mut gizmos: Gizmos,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        info!("toggle gizmos");
        // for (_, config, _) in config_store.iter_mut() {
        //     config.depth_bias = if config.depth_bias == 0. { -1. } else { 0. };
        // }
    }

    // let (config, _) = config_store.config_mut::<MyRoundGizmos>();
    // if keyboard.pressed(KeyCode::ArrowRight) {
    //     config.line.width += 5. * time.delta_secs();
    //     config.line.width = config.line.width.clamp(0., 50.);
    // }
    // if keyboard.pressed(KeyCode::ArrowLeft) {
    //     config.line.width -= 5. * time.delta_secs();
    //     config.line.width = config.line.width.clamp(0., 50.);
    // }
    // if keyboard.just_pressed(KeyCode::Digit1) {
    //     config.enabled ^= true;
    // }

    for vehicle in vehicles.iter_mut() {
        gizmos.cross(
            Vec3::new(vehicle.position_target.x, 0.0, vehicle.position_target.y),
            5.0,
            BLUE_VIOLET,
        );
        gizmos.sphere(
            Vec3::new(vehicle.position_current.x, 0.0, vehicle.position_current.y),
            2.0,
            GREEN_YELLOW,
        );
    }
}
