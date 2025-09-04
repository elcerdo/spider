use crate::global_state::{GlobalState, TRACK_NICKNAMES, TrackNickname};
use crate::material::racing_line_material;
use crate::track::{TRACK_HANDLES, Track};

use super::colors::*;

use bevy::math::{Affine2, Vec2};
use bevy::pbr::{StandardMaterial, UvChannel};
use bevy::prelude::*;
use bevy::sprite::Anchor;

use std::f32::consts::PI;

const LOGO_PATH: &str = "textures/super_splash_logo.png";

//////////////////////////////////////////////////////////////////////
pub struct TrackSelectionMenuPlugin;

impl Plugin for TrackSelectionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GlobalState::TrackSelectionInit), populate_scene);

        for track_nickname in TRACK_NICKNAMES {
            let state = GlobalState::TrackSelectionHoovered(*track_nickname);
            let state_ = GlobalState::TrackSelected(*track_nickname);
            app.add_systems(OnEnter(state), update_selected_model);
            app.add_systems(OnEnter(state), update_logo_transform);
            //     app.add_systems(Update, quit_with_escape.run_if(in_state(state)));
            app.add_systems(OnEnter(state_), depopulate_all);
        }

        app.add_systems(Update, animate_selected_model);
        app.add_systems(Update, update_menu);
        // app.add_systems(
        //     Update,
        //     quit_with_escape.run_if(in_state(GlobalState::TrackSelectionIdle)),
        // );
    }
}

// fn quit_with_escape(mut writer: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
//     if keyboard.just_pressed(KeyCode::Escape) {
//         writer.write(AppExit::Success);
//     }
// }

//////////////////////////////////////////////////////////////////////

#[derive(Component)]
struct TrackSelectionModelMarker;

fn update_selected_model(
    mut commands: Commands,
    entities: Query<(Entity, &Transform), With<TrackSelectionModelMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut racing_line_materials: ResMut<Assets<racing_line_material::RacingLineMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    tracks: Res<Assets<Track>>,
    state: Res<State<GlobalState>>,
    asset_server: Res<AssetServer>,
) {
    use bevy::image::ImageAddressMode;
    use bevy::image::ImageLoaderSettings;
    use bevy::image::ImageSampler;
    use bevy::image::ImageSamplerDescriptor;
    use racing_line_material::AnimatedRacingLineMarker;

    let mut rotation = Quat::IDENTITY;
    for (entity, transform) in entities {
        rotation = transform.rotation;
        commands.entity(entity).despawn();
    }

    let (track, mesh) = match state.get() {
        GlobalState::TrackSelectionHoovered(TrackNickname::Beginner) => {
            let track = tracks.get(&TRACK_HANDLES[0]).unwrap();
            (track, meshes.add(track.track.clone()))
        }
        GlobalState::TrackSelectionHoovered(TrackNickname::Vertical) => {
            let track = tracks.get(&TRACK_HANDLES[1]).unwrap();
            (track, meshes.add(track.track.clone()))
        }
        GlobalState::TrackSelectionHoovered(TrackNickname::Advanced) => {
            let track = tracks.get(&TRACK_HANDLES[2]).unwrap();
            (track, meshes.add(track.track.clone()))
        }
        _ => unreachable!(),
    };

    // materials
    let make_tileable = |settings: &mut ImageLoaderSettings| {
        *settings = ImageLoaderSettings {
            // is_srgb: false,
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..ImageSamplerDescriptor::default()
            }),
            ..ImageLoaderSettings::default()
        }
    };
    let checkerboard_material = standard_materials.add(StandardMaterial {
        base_color_channel: UvChannel::Uv1,
        base_color_texture: Some(
            asset_server.load_with_settings("textures/uv_checker_bw.png", make_tileable),
        ),
        uv_transform: Affine2::from_scale(Vec2::new(1.0 / 8.0, 1.0 / 8.0)),
        ..StandardMaterial::default()
    });
    let _tiledflow_material = standard_materials.add(StandardMaterial {
        base_color_channel: UvChannel::Uv0,
        base_color_texture: Some(
            asset_server.load_with_settings("textures/panel-border-010.png", make_tileable),
        ),
        ..StandardMaterial::default()
    });
    let racing_line_material = racing_line_materials.add(racing_line_material::make(
        &asset_server,
        track.total_length,
    ));

    // models
    commands.spawn((
        TrackSelectionModelMarker,
        Mesh3d(mesh.clone()),
        MeshMaterial3d(checkerboard_material),
        AnimatedRacingLineMarker,
        Transform::from_rotation(rotation),
    ));
    commands.spawn((
        TrackSelectionModelMarker,
        Mesh3d(mesh),
        MeshMaterial3d(racing_line_material),
        AnimatedRacingLineMarker,
        Transform::from_rotation(rotation) * Transform::from_translation(track.initial_up * 1e-3),
    ));
}

fn animate_selected_model(
    query: Query<&mut Transform, With<TrackSelectionModelMarker>>,
    time: Res<Time>,
) {
    for mut transform in query {
        transform.rotation *= Quat::from_axis_angle(Vec3::Y, 0.1 * PI * time.delta_secs());
    }
}

//////////////////////////////////////////////////////////////////////

#[derive(Component)]
struct TrackSelectionSceneMarker;

fn populate_scene(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GlobalState>>,
    asset_server: Res<AssetServer>,
) {
    use bevy::prelude::*;

    // light
    commands.spawn((
        TrackSelectionSceneMarker,
        DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            illuminance: light_consts::lux::OVERCAST_DAY,
            ..default()
        },
        Transform::from_translation(Vec3::Y).looking_at(vec3(-1.0, 0.0, -1.0), Vec3::Y),
    ));

    // camera
    commands.spawn((
        TrackSelectionSceneMarker,
        Camera3d::default(),
        Transform::from_xyz(-20.0, 20.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        TrackSelectionSceneMarker,
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
    ));

    // ui buttons
    commands
        .spawn((
            TrackSelectionSceneMarker,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Node::default()
            },
        ))
        .with_children(|parent| {
            let mut add_button = |track_nickname: TrackNickname| {
                parent
                    .spawn((
                        track_nickname,
                        Button,
                        Node {
                            width: Val::Px(170.0),
                            height: Val::Px(60.0),
                            border: UiRect::all(Val::Px(5.0)),
                            margin: UiRect::right(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor(COLOR_UI_FG.into()),
                        BorderRadius::MAX,
                        BackgroundColor(COLOR_UI_BG.into()),
                    ))
                    .with_child((
                        Text::new(format!("{:?}", track_nickname)),
                        TextFont {
                            font_size: 25.0,
                            ..TextFont::default()
                        },
                        TextColor(COLOR_UI_FG.into()),
                    ));
            };

            add_button(TrackNickname::Beginner);
            add_button(TrackNickname::Vertical);
            add_button(TrackNickname::Advanced);
        });

    // logo
    commands.spawn((
        TrackSelectionSceneMarker,
        Sprite::from_image(asset_server.load(LOGO_PATH)),
    ));

    next_state.set(GlobalState::TrackSelectionIdle);
}

fn update_menu(
    mut query: Query<
        (&Interaction, &TrackNickname, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GlobalState>>,
) {
    for (interaction, _, mut bg_color) in &mut query {
        if *interaction == Interaction::None {
            *bg_color = COLOR_UI_BG.into();
            next_state.set(GlobalState::TrackSelectionIdle);
        }
    }
    for (interaction, track, mut bg_color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = COLOR_UI_PRESSED.into();
                next_state.set(GlobalState::TrackSelected(*track));
            }
            Interaction::Hovered => {
                *bg_color = COLOR_UI_HOOVER.into();
                next_state.set(GlobalState::TrackSelectionHoovered(*track));
            }
            _ => {}
        }
    }
}

fn update_logo_transform(
    query: Query<(&mut Transform, &mut Sprite), With<TrackSelectionSceneMarker>>,
    window: Single<&Window>,
) {
    let transform_ = Vec3::new(
        window.physical_width() as f32 - 20.0,
        window.physical_height() as f32 - 30.0,
        0.0,
    ) / 2.0;
    let transform_ = Transform::from_translation(transform_).with_scale(Vec3::ONE * 0.5);
    for (mut transform, mut sprite) in query {
        *transform = transform_;
        sprite.anchor = Anchor::TopRight;
    }
}

//////////////////////////////////////////////////////////////////////

fn depopulate_all(
    mut commands: Commands,
    entities_aa: Query<Entity, With<TrackSelectionModelMarker>>,
    entities_bb: Query<Entity, With<TrackSelectionSceneMarker>>,
) {
    for entity in entities_aa {
        commands.entity(entity).despawn();
    }
    for entity in entities_bb {
        commands.entity(entity).despawn();
    }
}
