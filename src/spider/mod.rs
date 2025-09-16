mod anim;
mod data;
mod gizmos;
mod gun;
mod leg;
mod physics;
mod theme;

use super::global_state::GlobalState;

use bevy::prelude::*;

use bevy::color::palettes::tailwind::*;
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
                gun::update_guns,
                anim::update_animations,
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
        for mut vehicle in vehicles.iter_mut() {
            vehicle.reset();
        }
    }
}

fn populate_spiders(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    use data::Controller;

    // Load animations from asset.
    let anim_idle = asset_server.load(GltfAssetLabel::Animation(0).from_asset(MODEL_SPIDER_PATH));
    let anim_shoot = asset_server.load(GltfAssetLabel::Animation(2).from_asset(MODEL_SPIDER_PATH));

    // Create the graph.
    let mut animation_graph = AnimationGraph::new();
    let node_idle = animation_graph.add_clip(anim_idle, 1.0, animation_graph.root);
    let node_shoot = animation_graph.add_clip(anim_shoot, 1.0, animation_graph.root);
    let animation_graph = animation_graphs.add(animation_graph);

    let scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset(MODEL_SPIDER_PATH));

    let mut populate_spider = |pos: Vec2,
                               angle: f32,
                               controller: Controller,
                               color_aa: LinearRgba,
                               color_bb: LinearRgba| {
        let mut scene = commands.spawn((
            SceneRoot(scene.clone()),
            data::SpiderVehicle::new(pos, angle, controller),
            data::SpiderAnimation {
                node_idle,
                node_shoot,
                graph: animation_graph.clone(),
            },
            data::SpiderTheme { color_aa, color_bb },
            data::SpiderLegs::default(),
            data::SpiderGun::default(),
            Transform::IDENTITY,
        ));

        scene.observe(leg::populate_legs);
        scene.observe(anim::populate_animations);
        scene.observe(theme::set_theme);

        #[cfg(feature = "debug_gizmos")]
        scene.observe(gizmos::add_reference_axis);
    };

    populate_spider(
        -Vec2::X * 10.0,
        -PI / 2.0,
        Controller::Gamepad,
        BLUE_500.into(),
        ORANGE_500.into(),
    );

    populate_spider(
        Vec2::X * 10.0,
        PI / 2.0,
        Controller::Keyboard,
        YELLOW_500.into(),
        RED_500.into(),
    );
}
