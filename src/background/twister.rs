use super::BackgroundMarker;

use bevy::prelude::*;
use bevy::render::{
    mesh::{
        Indices, PrimitiveTopology, VertexAttributeValues,
        skinning::{SkinnedMesh, SkinnedMeshInverseBindposes},
    },
    render_asset::RenderAssetUsages,
};

use bevy::color::palettes::css::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct AnimatedTwister;

pub fn populate(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut inverse_bindposes_assets: ResMut<Assets<SkinnedMeshInverseBindposes>>,
) {
    info!("** populate twister **");

    // Create joint entities
    let joint_0 = commands
        .spawn((
            BackgroundMarker,
            Transform::from_xyz(10.0, 0.0, -10.0),
            // Transform::from_scale(Vec3::ONE * 5.0).with_translation(vec3(10.0, 0.0, -10.0)),
        ))
        .id();
    let joint_1 = commands
        .spawn((
            BackgroundMarker,
            AnimatedTwister,
            Transform::from_xyz(10.0, 10.0, -10.0),
        ))
        .id();

    let joint_entities = vec![joint_0, joint_1];

    // Create inverse bindpose matrices for a skeleton consists of 2 joints
    // let center = Vec3::new(-0.5, -1.0, 0.0);
    let inverse_bindposes = inverse_bindposes_assets.add(vec![
        Mat4::from_diagonal(vec4(10.0, 1.0, 1.0, 1.0)),
        Mat4::from_diagonal(vec4(10.0, 1.0, 1.0, 1.0))
            * Mat4::from_translation(vec3(-0.5, -2.0, 0.0)),
    ]);

    // Create a mesh
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    // Set mesh vertex positions
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.5, 0.0],
            [1.0, 0.5, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.5, 0.0],
            [1.0, 1.5, 0.0],
            [0.0, 2.0, 0.0],
            [1.0, 2.0, 0.0],
        ],
    )
    // Add UV coordinates that map the left half of the texture since its a 1 x
    // 2 rectangle.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [0.0, 0.00],
            [0.5, 0.00],
            [0.0, 0.25],
            [0.5, 0.25],
            [0.0, 0.50],
            [0.5, 0.50],
            [0.0, 0.75],
            [0.5, 0.75],
            [0.0, 1.00],
            [0.5, 1.00],
        ],
    )
    // Set mesh vertex normals
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 10])
    // Set mesh vertex joint indices for mesh skinning.
    // Each vertex gets 4 indices used to address the `JointTransforms` array in the vertex shader
    //  as well as `SkinnedMeshJoint` array in the `SkinnedMesh` component.
    // This means that a maximum of 4 joints can affect a single vertex.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_JOINT_INDEX,
        // Need to be explicit here as [u16; 4] could be either Uint16x4 or Unorm16x4.
        VertexAttributeValues::Uint16x4(vec![
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
        ]),
    )
    // Set mesh vertex joint weights for mesh skinning.
    // Each vertex gets 4 joint weights corresponding to the 4 joint indices assigned to it.
    // The sum of these weights should equal to 1.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_JOINT_WEIGHT,
        vec![
            [1.00, 0.00, 0.0, 0.0],
            [1.00, 0.00, 0.0, 0.0],
            [0.75, 0.25, 0.0, 0.0],
            [0.75, 0.25, 0.0, 0.0],
            [0.50, 0.50, 0.0, 0.0],
            [0.50, 0.50, 0.0, 0.0],
            [0.25, 0.75, 0.0, 0.0],
            [0.25, 0.75, 0.0, 0.0],
            [0.00, 1.00, 0.0, 0.0],
            [0.00, 1.00, 0.0, 0.0],
        ],
    )
    // Tell bevy to construct triangles from a list of vertex indices,
    // where each 3 vertex indices form a triangle.
    .with_inserted_indices(Indices::U16(vec![
        0, 1, 3, 0, 3, 2, 2, 3, 5, 2, 5, 4, 4, 5, 7, 4, 7, 6, 6, 7, 9, 6, 9, 8,
    ]));

    let mesh = meshes.add(mesh);

    // Create skinned mesh renderer. Note that its transform doesn't affect the position of the mesh.

    commands.spawn((
        BackgroundMarker,
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: WHITE_SMOKE.into(),
            base_color_texture: Some(asset_server.load("textures/uv_checker_bw.png")),
            ..default()
        })),
        SkinnedMesh {
            inverse_bindposes: inverse_bindposes.clone(),
            joints: joint_entities,
        },
    ));
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<AnimatedTwister>>,
    mut gizmos: Gizmos,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_y(PI / 2.0 * ops::sin(10.0 * time.elapsed_secs()));
        gizmos.axes(*transform, 1.0);
    }
}
