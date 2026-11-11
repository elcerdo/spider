use crate::global_state::GlobalState;
use crate::material::parallax_material;
use crate::spider::{BBoxPrimitive, StaticPrimitives};

use bevy::prelude::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StaticPrimitives>();
        app.add_systems(OnEnter(GlobalState::Ready), populate);
        app.add_systems(OnExit(GlobalState::Ready), depopulate);
    }
}

#[derive(Component)]
struct ObstaclesMarker;

fn depopulate(
    mut commands: Commands,
    query: Query<Entity, With<ObstaclesMarker>>,
    mut primitives: ResMut<StaticPrimitives>,
) {
    primitives.0.clear();
    for entity in query {
        commands.entity(entity).despawn();
    }
}

fn populate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut primitives: ResMut<StaticPrimitives>,
) {
    let brick_wall_material =
        materials.add(parallax_material::make(&asset_server, 1.0, "brick_wall"));
    let mut insert_low_wall = |position: Vec2, brick_wall_size: Vec3| {
        let wall_entity = commands
            .spawn((
                ObstaclesMarker,
                Mesh3d(meshes.add(make_brick_wall_mesh(brick_wall_size))),
                MeshMaterial3d(brick_wall_material.clone()),
                Transform::from_xyz(position.x, brick_wall_size.y / 2.0, position.y),
            ))
            .id();
        primitives.0.push(BBoxPrimitive::WallLow {
            entity: wall_entity,
            brick_wall_size,
        });
    };

    insert_low_wall(Vec2::new(-4.0, 12.0), Vec3::new(10.0, 1.5, 0.5));
    insert_low_wall(Vec2::new(16.0, 12.0), Vec3::new(10.0, 1.5, 0.5));
    insert_low_wall(Vec2::new(-16.0, -12.0), Vec3::new(10.0, 1.5, 0.5));
    insert_low_wall(Vec2::new(8.0, -16.0), Vec3::new(10.0, 1.5, 0.5));
    insert_low_wall(Vec2::new(-24.0, 6.0), Vec3::new(0.5, 1.5, 10.0));
    insert_low_wall(Vec2::new(24.0, -6.0), Vec3::new(0.5, 1.5, 10.0));
}

fn make_brick_wall_mesh(size: Vec3) -> Mesh {
    use bevy::mesh::VertexAttributeValues;

    let mut mesh = Mesh::from(Cuboid::from_size(size));
    let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    else {
        unreachable!("Cuboid positions are Float32x3");
    };
    let Some(VertexAttributeValues::Float32x3(normals)) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
    else {
        unreachable!("Cuboid normals are Float32x3");
    };
    let half_size = size * 0.5;
    let uvs = positions
        .iter()
        .zip(normals)
        .map(|(position, normal)| {
            let [x, y, z] = *position;
            let u = if normal[0].abs() > 0.5 {
                if normal[0] > 0.0 {
                    half_size.z - z
                } else {
                    z + half_size.z
                }
            } else if normal[2] < 0.0 {
                half_size.x - x
            } else {
                x + half_size.x
            };
            let v = if normal[1].abs() > 0.5 {
                if normal[1] > 0.0 {
                    // Use the upper course as a single cap row across the wall's thickness.
                    size.y * (0.75 + 0.25 * (half_size.z - z) / size.z)
                } else {
                    z + half_size.z
                }
            } else {
                y + half_size.y
            };
            // One texture tile contains four brick rows and spans the wall's height.
            [u / size.y, v / size.y]
        })
        .collect::<Vec<_>>();

    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_1, uvs);
    mesh.with_generated_tangents().unwrap()
}
