mod data;
mod physics;

use super::global_state::GlobalState;
use bevy::math::NormedVectorSpace;
use data::SpiderData;
use physics::lift;

use bevy::scene::SceneInstanceReady;

use std::collections::BTreeMap;

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
                update_spider_legs,
                display_gizmos,
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

#[derive(Clone, Debug)]
struct SpiderLeg {
    parent: Entity,
    marker: Entity,
    entity: Entity,
}

#[derive(Component)]
struct SpiderAnimation {
    graph: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
    legs: BTreeMap<(String, String), SpiderLeg>,
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

    let mut scene = commands.spawn((
        SceneRoot(scene.clone()),
        SpiderData::from_position_and_angle(Vec2::ZERO, -PI / 2.0),
        SpiderAnimation {
            graph,
            index,
            legs: BTreeMap::new(),
        },
        Transform::IDENTITY,
    ));

    scene.observe(populate_legs);
    scene.observe(play_animation_when_ready);

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

const SPIDER_LEG_LENGTH: f32 = 3.5;
const SPIDER_STEP_LENGTH: f32 = 1.0;
const SPIDER_STEP_LEAD: f32 = 0.25;

fn populate_legs(
    trigger: Trigger<SceneInstanceReady>,
    mut animation: Single<&mut SpiderAnimation>,
    children: Query<&Children>,
    names: Query<&Name>,
    parents: Query<&ChildOf>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("** populate legs **");

    let re = regex::Regex::new(r"^leg_(left|right)_(front|mid|back)$").unwrap();
    let target = trigger.target();

    let mesh = Cuboid::new(0.5, 0.5, SPIDER_LEG_LENGTH);
    let material = StandardMaterial {
        base_color: RED.into(),
        emissive: RED.into(),
        ..default()
    };
    let mesh = meshes.add(mesh);
    let material = materials.add(material);

    for entity in children.iter_descendants(target) {
        if let Ok(entity_name) = names.get(entity) {
            if let Some(groups) = re.captures(entity_name) {
                let key: (String, String) = (groups[1].into(), groups[2].into());

                let mut marker =
                    commands.spawn((InheritedVisibility::VISIBLE, Transform::IDENTITY));

                marker.with_child((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(0.0, 0.0, SPIDER_LEG_LENGTH / 2.0),
                ));

                let marker = marker.id();

                let ChildOf(parent) = parents.get(entity).unwrap();
                let parent = *parent;

                let value = SpiderLeg {
                    parent,
                    marker,
                    entity,
                };

                let parent_name = names.get(parent).unwrap();
                info!(
                    "{:?} -> ({}, {:?}, {})",
                    key.clone(),
                    parent_name,
                    marker,
                    entity_name,
                );

                animation.legs.insert(key.clone(), value.clone());
            }
        }
    }

    assert!(animation.legs.len() == 6);

    for leg in animation.legs.values() {
        let mut leg_commands = commands.entity(leg.entity);
        leg_commands.remove_parent_in_place();
        leg_commands.set_parent_in_place(leg.marker);
    }
}

fn update_spider_legs(
    animations: Query<&SpiderAnimation>,
    global_transforms: Query<&GlobalTransform>,
    mut transforms: Query<&mut Transform>,
) {
    assert!(SPIDER_STEP_LEAD < SPIDER_STEP_LENGTH);
    for animation in animations.iter() {
        for leg in animation.legs.values() {
            let transform = global_transforms.get(leg.parent).unwrap();
            let pos = transform.transform_point(Vec3::Y * SPIDER_LEG_LENGTH);
            let pos__ = transform.transform_point(Vec3::ZERO);
            assert!((pos__ - transform.translation()).norm() < 1e-5);

            let mut transform_ = transforms.get_mut(leg.marker).unwrap();
            let pos_ = transform_.transform_point(Vec3::ZERO);
            assert!((pos_ - transform_.translation).norm() < 1e-5);

            let delta = pos - pos_;
            if delta.norm() > SPIDER_STEP_LENGTH {
                let lead = delta.normalize() * SPIDER_STEP_LEAD;
                transform_.translation = pos + lead;
            }

            let delta = pos__ - pos_;
            let angle = delta.zx().to_angle();
            transform_.rotation = Quat::from_axis_angle(Vec3::Y, angle);

            let mut transform__ = transforms.get_mut(leg.entity).unwrap();
            transform__.translation.y = 0.5;
            transform__.translation.z = 3.0;
        }
    }
}

fn display_gizmos(
    vehicles_nad_animations: Query<(&SpiderData, &SpiderAnimation)>,
    global_transforms: Query<&GlobalTransform>,
    mut gizmos: Gizmos,
) {
    for (vehicle, animation) in vehicles_nad_animations.iter() {
        gizmos.cross(lift(vehicle.position_target), 5.0, BLUE_VIOLET);
        gizmos.sphere(lift(vehicle.position_current), 2.0, GREEN_YELLOW);
        for leg in animation.legs.values() {
            let transform = global_transforms.get(leg.parent).unwrap();
            let pos = transform.transform_point(Vec3::Y * SPIDER_LEG_LENGTH);
            let pos__ = transform.transform_point(Vec3::ZERO);
            assert!((pos__ - transform.translation()).norm() < 1e-5);
            gizmos.arrow(pos__, pos, WHITE);
        }
    }
}
