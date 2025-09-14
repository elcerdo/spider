mod anim;
mod data;
mod gizmos;
mod leg;
mod physics;

use super::global_state::GlobalState;

use bevy::prelude::*;

use std::f32::consts::PI;

const MODEL_SPIDER_PATH: &str = "models/tachikoma.glb";

//////////////////////////////////////////////////////////////////////

pub struct SpiderPlugin;

impl Plugin for SpiderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (populate_spider).chain());
        app.add_systems(
            Update,
            (
                reset_vehicles,
                physics::update_vehicles,
                leg::update_legs,
                gizmos::display_vehicles,
                gizmos::display_legs,
                // collision::bounce_and_resolve_checkpoints,
                // update_statuses,
                // update_boards_and_cups,
            )
                .chain()
                .run_if(in_state(GlobalState::Ready)),
        );
    }
}

//////////////////////////////////////////////////////////////////////

fn reset_vehicles(
    mut vehicles: Query<&mut data::SpiderVehicle>,
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
    server: Res<AssetServer>,
    mut commands: Commands,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // animation from our example asset, which has an index of two.
    let (graph, index) = AnimationGraph::from_clip(
        server.load(GltfAssetLabel::Animation(0).from_asset(MODEL_SPIDER_PATH)),
    );
    let graph: Handle<AnimationGraph> = graphs.add(graph);

    let scene: Handle<Scene> = server.load(GltfAssetLabel::Scene(0).from_asset(MODEL_SPIDER_PATH));

    let mut scene = commands.spawn((
        SceneRoot(scene.clone()),
        data::SpiderVehicle::from_position_and_angle(Vec2::ZERO, -PI / 2.0),
        data::SpiderAnimation { graph, index },
        data::SpiderLegs::default(),
        Transform::IDENTITY,
    ));

    scene.observe(leg::populate_legs);
    scene.observe(anim::play_idle);
    #[cfg(feature = "debug_gizmos")]
    scene.observe(add_reference_axis);
}

#[cfg(feature = "debug_gizmos")]
fn add_reference_axis(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    let target = trigger.target();
    commands.entity(target).with_children(|parent| {
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
