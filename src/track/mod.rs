use crate::global_state::{GlobalState, TRACK_NICKNAMES, TrackNickname};
use crate::material::racing_line_material;
use crate::material::wavy_material;

use bevy::asset::{AssetServer, Assets};
use bevy::color::Srgba;
use bevy::pbr::{ScreenSpaceReflections, StandardMaterial};
use bevy::render::mesh::Mesh;

use bevy::prelude::info;
use bevy::prelude::{Commands, Res, ResMut};
use bevy::prelude::{Component, Entity, Query, With};
use bevy::prelude::{NextState, State};

mod data;
mod piece;

pub use data::TRACK_HANDLES;
// pub use piece::Segment;
pub use piece::Track;

const TRACK_GROUND_COLOR: Srgba = Srgba::rgb(0.4, 0.4, 0.4);
const TRACK_EPSILON: f32 = 5e-2;

//////////////////////////////////////////////////////////////////////

pub struct TrackPlugin;

impl bevy::prelude::Plugin for TrackPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        use bevy::prelude::*;
        app.init_asset::<Track>();
        app.add_systems(PreStartup, data::prepare_tracks);

        for track_nickname in TRACK_NICKNAMES {
            let state = GlobalState::TrackSelected(*track_nickname);
            let state_ = GlobalState::InGame(*track_nickname);
            app.add_systems(OnEnter(state), (populate_track, to_game).chain());
            app.add_systems(OnEnter(state_), populate_camera_and_lights);
            app.add_systems(OnExit(state_), depopulate_all);
        }
    }
}

//////////////////////////////////////////////////////////////////////

#[derive(Component)]
struct GameSceneMarker;

fn depopulate_all(mut commands: Commands, query: Query<Entity, With<GameSceneMarker>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

fn populate_camera_and_lights(mut commands: Commands, asset_server: Res<AssetServer>) {
    use bevy::prelude::*;
    use bevy::render::camera::ScalingMode;

    info!("** populate_camera_and_lights **");

    // lights
    commands.spawn((
        GameSceneMarker,
        PointLight {
            shadows_enabled: true,
            intensity: 8e6,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_translation(Vec3::Y * 20.0),
    ));
    // commands.spawn((
    //     GameSceneMarker,
    //     DirectionalLight {
    //         color: Color::WHITE,
    //         shadows_enabled: true,
    //         illuminance: light_consts::lux::OVERCAST_DAY,
    //         ..default()
    //     },
    //     Transform::from_translation(Vec3::Y).looking_at(Vec3::new(-1.0, 0.0, -1.0), Vec3::Y),
    // ));
    // commands.spawn((
    //     GameSceneMarker,
    //     DirectionalLight {
    //         color: Color::WHITE,
    //         shadows_enabled: true,
    //         illuminance: light_consts::lux::OVERCAST_DAY,
    //         ..default()
    //     },
    //     Transform::from_translation(Vec3::Y).looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
    // ));

    // camera
    commands.spawn((
        GameSceneMarker,
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 14.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Msaa::Off,
        ScreenSpaceReflections::default(),
        // EnvironmentMapLight {
        //     diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
        //     specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        //     intensity: 10e2,
        //     ..default()
        // },
        Transform::from_xyz(-10.0, 10.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn populate_track(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut racing_line_materials: ResMut<Assets<racing_line_material::RacingLineMaterial>>,
    tracks: Res<Assets<Track>>,
    asset_server: Res<AssetServer>,
    state: Res<State<GlobalState>>,
) {
    use racing_line_material::AnimatedRacingLineMarker;
    use wavy_material::AnimatedWavyMarker;

    use bevy::color::Color;
    use bevy::prelude::*;

    info!("** populate_track **");

    let (track_length, track_up, maybe_lateral_range, track_mesh /*, checkpoint_mesh*/) =
        match state.get() {
            GlobalState::TrackSelected(TrackNickname::Beginner) => {
                let track = tracks.get(&TRACK_HANDLES[0]).unwrap();
                (
                    track.total_length,
                    track.initial_up,
                    None,
                    meshes.add(track.track.clone()),
                    // meshes.add(track.checkpoint.clone()),
                )
            }
            GlobalState::TrackSelected(TrackNickname::Vertical) => {
                let track = tracks.get(&TRACK_HANDLES[1]).unwrap();
                (
                    track.total_length,
                    track.initial_up,
                    None,
                    meshes.add(track.track.clone()),
                    // meshes.add(track.checkpoint.clone()),
                )
            }
            GlobalState::TrackSelected(TrackNickname::Advanced) => {
                let track = tracks.get(&TRACK_HANDLES[2]).unwrap();
                (
                    track.total_length,
                    track.initial_up,
                    Some(Vec2::new(-1.5, 1.5)),
                    meshes.add(track.track.clone()),
                    // meshes.add(track.checkpoint.clone()),
                )
            }
            _ => unreachable!(),
        };

    // materials
    let checkpoint_material = standard_materials.add(StandardMaterial {
        base_color: Color::hsva(0.0, 0.8, 1.0, 0.8),
        ..StandardMaterial::default()
    });
    let wavy_material = standard_materials.add(wavy_material::make(&asset_server));
    let mut overlay_material = racing_line_material::make(&asset_server, track_length);
    overlay_material.middle_line_width = -1.0; // no middle line
    if let Some(lateral_range) = maybe_lateral_range {
        overlay_material.lateral_range = lateral_range;
    }
    let overlay_material = racing_line_materials.add(overlay_material);

    // // checkpoints
    // commands.spawn((
    //     GameSceneMarker,
    //     Mesh3d(checkpoint_mesh),
    //     MeshMaterial3d(checkpoint_material),
    //     Transform::from_translation(2.0 * TRACK_EPSILON * track_up),
    // ));

    // racing lines
    commands.spawn((
        GameSceneMarker,
        Mesh3d(track_mesh.clone()),
        AnimatedRacingLineMarker,
        MeshMaterial3d(overlay_material),
        Transform::from_translation(1.0 * TRACK_EPSILON * track_up),
    ));

    // track
    commands.spawn((
        GameSceneMarker,
        Mesh3d(track_mesh),
        AnimatedWavyMarker,
        MeshMaterial3d(wavy_material.clone()),
        Transform::from_translation(0.0 * TRACK_EPSILON * track_up),
    ));

    // ground plane
    commands.spawn((
        GameSceneMarker,
        Mesh3d(meshes.add(Plane3d::default().mesh().size(30.0, 40.0).subdivisions(20))),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            perceptual_roughness: 1.0,
            metallic: 0.0,
            base_color: TRACK_GROUND_COLOR.into(),
            ..default()
        })),
        Transform::from_translation(-1.0 * TRACK_EPSILON * track_up),
    ));
}

fn to_game(state: Res<State<GlobalState>>, mut next_state: ResMut<NextState<GlobalState>>) {
    match state.get() {
        GlobalState::TrackSelected(track_nickname) => {
            next_state.set(GlobalState::InGame(*track_nickname));
        }
        _ => unreachable!(),
    };
}
