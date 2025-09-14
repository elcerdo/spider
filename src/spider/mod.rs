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
        app.add_systems(Startup, populate_spiders);
        app.add_systems(
            Update,
            (
                reset_vehicles,
                physics::update_vehicles,
                leg::update_legs,
                gizmos::display_vehicles,
                gizmos::display_legs,
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

fn populate_spiders(
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

    let mut populate_spider = |pos: Vec2, angle: f32, keyboard_or_gamepad: bool| {
        let mut scene = commands.spawn((
            SceneRoot(scene.clone()),
            data::SpiderVehicle::new(pos, angle, keyboard_or_gamepad),
            data::SpiderAnimation {
                graph: graph.clone(),
                index: index.clone(),
            },
            data::SpiderLegs::default(),
            Transform::IDENTITY,
        ));

        scene.observe(leg::populate_legs);
        scene.observe(anim::play_idle);
        #[cfg(feature = "debug_gizmos")]
        scene.observe(gizmos::add_reference_axis);
    };

    populate_spider(-Vec2::X * 10.0, -PI / 2.0, false);
    populate_spider(Vec2::X * 10.0, PI / 2.0, true);
}
