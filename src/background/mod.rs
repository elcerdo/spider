mod twister;

use crate::global_state::GlobalState;
use crate::material::parallax_material;

use bevy::prelude::*;

//////////////////////////////////////////////////////////////////////

pub struct BackgroundPlugin;

impl bevy::prelude::Plugin for BackgroundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        use bevy::prelude::*;

        {
            let state = GlobalState::Ready;
            app.add_systems(
                OnEnter(state),
                (
                    populate_background,
                    populate_lights_and_cameras,
                    twister::populate,
                )
                    .chain(),
            );
            app.add_systems(OnExit(state), depopulate_background);
            app.add_systems(Update, twister::animate.run_if(in_state(state)));
        }
    }
}

//////////////////////////////////////////////////////////////////////

#[derive(Component)]
struct BackgroundMarker;

fn depopulate_background(mut commands: Commands, query: Query<Entity, With<BackgroundMarker>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

fn populate_lights_and_cameras(mut commands: Commands) {
    // light
    commands.spawn((
        BackgroundMarker,
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
        BackgroundMarker,
        Camera3d::default(),
        Transform::from_xyz(-20.0, 20.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        BackgroundMarker,
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
    ));
}

fn populate_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<bevy::render::mesh::Mesh>>,
    mut images: ResMut<Assets<bevy::image::Image>>,
    mut materials: ResMut<Assets<bevy::pbr::StandardMaterial>>,
    asset_server: Res<bevy::asset::AssetServer>,
) {
    use bevy::prelude::*;

    info!("** populate_background **");

    // tower
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(make_uv_debug_texture())),
        ..default()
    });
    commands.spawn((
        BackgroundMarker,
        Mesh3d(meshes.add(Cuboid::new(1.0, 5.0, 1.0))),
        MeshMaterial3d(debug_material),
        Transform::from_xyz(0.0, 2.5, -10.0),
    ));

    // cube
    commands.spawn((
        BackgroundMarker,
        Mesh3d(meshes.add(make_cube_mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/array_texture.png")),
            ..default()
        })),
        Transform::from_xyz(12.0, 0.5, -2.0),
    ));

    // parallal
    let parallal_material = materials.add(parallax_material::make(asset_server, 2.0));
    commands.spawn((
        BackgroundMarker,
        Mesh3d(
            meshes.add(
                Mesh::from(Cuboid::default())
                    .with_generated_tangents()
                    .unwrap(),
            ),
        ),
        MeshMaterial3d(parallal_material),
        Transform::from_xyz(3.0, 2.0, 18.0).with_scale(Vec3::ONE * 4.0),
    ));
}

//////////////////////////////////////////////////////////////////////

/*

use bevy::prelude::*;

const CAMERA_ROTATION: Quat = Quat::from_xyzw(-0.24781081, -0.2946635, -0.07941471, 0.91948706);
const CAMERA_DATA: &[(Transform, f32)] = &[
    (
        Transform {
            translation: Vec3::new(-10.0, 10.0, 14.0),
            rotation: CAMERA_ROTATION,
            scale: Vec3::ONE,
        },
        // 20 world units per pixel of window height.
        20.0,
    ),
    (
        Transform {
            translation: Vec3::new(-18.0, 10.0, 16.0),
            rotation: CAMERA_ROTATION,
            scale: Vec3::ONE,
        },
        // 14.0 world units per pixel of window height.
        14.0,
    ),
];

fn move_camera(
    mut camera_projection: Single<(&mut Transform, &mut Projection)>,
    mut current_view: Local<usize>,
    button: Res<ButtonInput<MouseButton>>,
) {
    use bevy::render::camera::OrthographicProjection;
    use bevy::render::camera::ScalingMode;

    if button.just_pressed(MouseButton::Left) {
        *current_view = (*current_view + 1) % CAMERA_DATA.len();
        bevy::prelude::info!("switched to camera {}", current_view.clone());
    }
    let target = CAMERA_DATA[*current_view];

    // update transform
    camera_projection.0.translation = camera_projection
        .0
        .translation
        .lerp(target.0.translation, 0.2);
    camera_projection.0.rotation = camera_projection.0.rotation.slerp(target.0.rotation, 0.2);

    // update projection scale
    let Projection::Orthographic(ortho_projection) = camera_projection.1.clone() else {
        return;
    };
    let ScalingMode::FixedVertical {
        mut viewport_height,
    } = ortho_projection.scaling_mode
    else {
        return;
    };
    viewport_height = viewport_height * 0.8 + target.1 * 0.2;
    *camera_projection.1 = Projection::from(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical { viewport_height },
        ..OrthographicProjection::default_3d()
    });
}

*/

//////////////////////////////////////////////////////////////////////

/// Creates a colorful test pattern
fn make_uv_debug_texture() -> bevy::image::Image {
    use bevy::render::render_asset::RenderAssetUsages;
    use bevy::render::render_resource::Extent3d;
    use bevy::render::render_resource::TextureDimension;
    use bevy::render::render_resource::TextureFormat;

    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    bevy::image::Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

#[rustfmt::skip]
fn make_cube_mesh() -> bevy::render::mesh::Mesh {
    use bevy::render::mesh::Indices;
    use bevy::render::mesh::Mesh;
    use bevy::render::render_asset::RenderAssetUsages;
    use bevy::render::render_resource::PrimitiveTopology;

    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // The camera coordinate space is right-handed x-right, y-up, z-back. This means "forward" is -Z.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y)
            [-0.5, 0.5, -0.5], // vertex with index 0
            [0.5, 0.5, -0.5], // vertex with index 1
            [0.5, 0.5, 0.5], // etc. until 23
            [-0.5, 0.5, 0.5],
            // bottom   (-y)
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // right    (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [0.5, 0.5, -0.5],
            // left     (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            // back     (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // forward  (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ],
    )
    // Set-up UV coordinates to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.2],
            // Assigning the UV coords for the bottom side.
            [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
            // Assigning the UV coords for the right side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the left side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the back side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
            // Assigning the UV coords for the forward side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
        ],
    )
    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    )
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    //
    // The first two defined triangles look like this (marked with the vertex indices,
    // and the axis), when looking down at the top (+y) of the cube:
    //   -Z
    //   ^
    // 0---1
    // |  /|
    // | / | -> +X
    // |/  |
    // 3---2
    //
    // The right face's (+x) triangles look like this, seen from the outside of the cube.
    //   +Y
    //   ^
    // 10--11
    // |  /|
    // | / | -> -Z
    // |/  |
    // 9---8
    //
    // The back face's (+z) triangles look like this, seen from the outside of the cube.
    //   +Y
    //   ^
    // 17--18
    // |\  |
    // | \ | -> +X
    // |  \|
    // 16--19
    .with_inserted_indices(Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ]))
}
