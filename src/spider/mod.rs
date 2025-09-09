mod data;
mod physics;

use data::SpiderData;

use bevy::scene::SceneInstanceReady;

use bevy::color::palettes::css::*;
use bevy::prelude::*;

const MODEL_SPIDER_PATH: &str = "models/tachikoma.glb";
// const MODEL_SPIDER_SCALE: f32 = 1.0;

//////////////////////////////////////////////////////////////////////

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

#[derive(Component)]
struct SpiderAnimation {
    graph: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}

fn populate_spider(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // tracks: Res<Assets<Track>>,
    // state: Res<State<GlobalState>>,
) {
    // animation from our example asset, which has an index of two.
    let (graph, index) = AnimationGraph::from_clip(
        server.load(GltfAssetLabel::Animation(0).from_asset(MODEL_SPIDER_PATH)),
    );
    let graph: Handle<AnimationGraph> = graphs.add(graph);

    let scene: Handle<Scene> = server.load(GltfAssetLabel::Scene(0).from_asset(MODEL_SPIDER_PATH));

    let mut scene = commands.spawn((
        SceneRoot(scene.clone()),
        SpiderData::from_position(Vec2::ZERO),
        SpiderAnimation { graph, index },
        Transform::from_translation(Vec3::Z * 5.0),
    ));

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
