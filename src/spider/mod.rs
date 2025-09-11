mod data;
mod physics;

use super::global_state::GlobalState;
use data::SpiderData;

use bevy::scene::SceneInstanceReady;

use bevy::color::palettes::css::*;
use bevy::prelude::*;

use std::f32::consts::PI;

const MODEL_SPIDER_PATH: &str = "models/tachikoma.glb";
// const MODEL_SPIDER_SCALE: f32 = 1.0;

//////////////////////////////////////////////////////////////////////

pub struct SpiderPlugin;

impl Plugin for SpiderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (populate_spider).chain());
        app.add_systems(
            Update,
            (
                reset_vehicle_positions,
                physics::update_vehicle_physics,
                update_gizmos,
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

#[derive(Component)]
struct SpiderAnimation {
    graph: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
    legs: Vec<Vec2>,
}

fn populate_spider(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // animation from our example asset, which has an index of two.
    let (graph, index) = AnimationGraph::from_clip(
        server.load(GltfAssetLabel::Animation(0).from_asset(MODEL_SPIDER_PATH)),
    );
    let graph: Handle<AnimationGraph> = graphs.add(graph);

    let scene: Handle<Scene> = server.load(GltfAssetLabel::Scene(0).from_asset(MODEL_SPIDER_PATH));

    let legs = vec![
        vec2(-5.0, 0.0),
        vec2(5.0, 0.0),
        vec2(0.0, -5.0),
        vec2(0.0, 5.0),
        vec2(5.0, 1.0),
    ];

    let mut scene = commands.spawn((
        SceneRoot(scene.clone()),
        SpiderData::from_position_and_angle(Vec2::ZERO, -PI / 2.0),
        SpiderAnimation { graph, index, legs },
        Transform::IDENTITY,
    ));

    scene.observe(enumerate_bones);
    // scene.observe(play_animation_when_ready);

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

fn enumerate_bones(
    trigger: Trigger<SceneInstanceReady>,
    // mut commands: Commands,
    // animations: Query<&SpiderAnimation>,
    children: Query<&Children>,
    names: Query<(&Name, &ChildOf)>,
    mut transforms: Query<(&mut Transform, &Name)>,
) {
    info!("** enumerate bones **");

    let re = regex::Regex::new(r"^leg_(left|right)_(front|mid|back)$").unwrap();

    let target = trigger.target();
    for entity in children.iter_descendants(target) {
        if let Ok((name, ChildOf(entity_))) = names.get(entity) {
            if let Some(groups) = re.captures(name) {
                let side = &groups[1];
                let side_ = &groups[2];
                if let Ok((mut transform, name_)) = transforms.get_mut(*entity_) {
                    // let pos = transform.transform_point(Vec3::ZERO);
                    transform.translation.z += 5.0;
                    let ww = match side {
                        "left" => 1.0,
                        "right" => -1.0,
                        _ => 0.0,
                    };
                    *transform = transform.with_rotation(
                        Quat::from_axis_angle(Vec3::Y, ww * PI / 6.0) * transform.rotation,
                    );
                    info!("entity {entity} -> {name} {side} {side_}");
                    info!("parent {entity_} -> name {name_} pos {transform:?}");
                }
            }
        }
    }
}

fn play_animation_when_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    animations: Query<&SpiderAnimation>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
) {
    info!("** starting animation **");

    // The entity we spawned in `setup_mesh_and_animation` is the trigger's target.
    // Start by finding the AnimationToPlay component we added to that entity.
    let target = trigger.target();
    if let Ok(animation) = animations.get(target) {
        // The SceneRoot component will have spawned the scene as a hierarchy
        // of entities parented to our entity. Since the asset contained a skinned
        // mesh and animations, it will also have spawned an animation player
        // component. Search our entity's descendants to find the animation player.
        for child in children.iter_descendants(target) {
            if let Ok(mut player) = players.get_mut(child) {
                // Tell the animation player to start the animation and keep
                // repeating it.
                //
                // If you want to try stopping and switching animations, see the
                // `animated_mesh_control.rs` example.
                player.play(animation.index).repeat();

                // Add the animation graph. This only needs to be done once to
                // connect the animation player to the mesh.
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation.graph.clone()));
            }
        }
    }
}

fn update_gizmos(query: Query<(&SpiderData, &SpiderAnimation)>, mut gizmos: Gizmos) {
    let lift = |aa: Vec2| -> Vec3 { vec3(aa.x, 0.0, aa.y) };

    for (vehicle, animation) in query.iter() {
        gizmos.cross(lift(vehicle.position_target), 5.0, BLUE_VIOLET);
        gizmos.sphere(lift(vehicle.position_current), 2.0, GREEN_YELLOW);
        for leg in animation.legs.iter() {
            // let leg = transform.transform_point(*leg);
            gizmos.arrow(lift(vehicle.position_current), lift(*leg), WHITE);
        }
    }
}
