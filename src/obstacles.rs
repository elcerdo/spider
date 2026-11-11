use crate::global_state::GlobalState;
use crate::material::parallax_material;

use bevy::prelude::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GlobalState::Ready), populate);
        app.add_systems(OnExit(GlobalState::Ready), depopulate);
    }
}

#[derive(Component)]
pub(crate) struct WallLow {
    pub brick_wall_size: Vec3,
}

#[derive(Component)]
pub(crate) struct WallHigh {
    pub brick_wall_size: Vec3,
}

fn depopulate(mut commands: Commands, query: Query<Entity, Or<(With<WallLow>, With<WallHigh>)>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

fn populate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let brick_wall_material =
        materials.add(parallax_material::make(&asset_server, 1.0, "brick_wall"));

    let mut insert_wall = |position: Vec2, brick_wall_size: Vec3, is_high: bool| {
        let mut wall_entity = commands.spawn((
            Mesh3d(meshes.add(make_brick_wall_mesh(brick_wall_size))),
            MeshMaterial3d(brick_wall_material.clone()),
            Transform::from_xyz(position.x, brick_wall_size.y / 2.0, position.y),
        ));
        if is_high {
            wall_entity.insert(WallHigh { brick_wall_size });
        } else {
            wall_entity.insert(WallLow { brick_wall_size });
        }
    };

    let mut insert_low_wall = |position, size| insert_wall(position, size, false);
    insert_low_wall(Vec2::new(-8.0, 25.0), Vec3::new(10.0, 1.5, 0.5));
    insert_low_wall(Vec2::new(16.0, 12.0), Vec3::new(10.0, 1.5, 0.5));
    insert_low_wall(Vec2::new(-16.0, -12.0), Vec3::new(10.0, 1.5, 0.5));
    insert_low_wall(Vec2::new(16.0, -28.0), Vec3::new(10.0, 1.5, 0.5));
    insert_low_wall(Vec2::new(-24.0, 6.0), Vec3::new(0.5, 1.5, 10.0));
    insert_low_wall(Vec2::new(24.0, -6.0), Vec3::new(0.5, 1.5, 10.0));

    let mut insert_high_wall = |position, size| insert_wall(position, size, true);
    insert_high_wall(Vec2::new(-4.0, 12.0), Vec3::new(12.0, 5.0, 0.5));
    insert_high_wall(Vec2::new(8.0, -16.0), Vec3::new(0.5, 5.0, 10.0));
}

fn make_brick_wall_mesh(size: Vec3) -> Mesh {
    use bevy::mesh::VertexAttributeValues;

    const BRICK_TEXTURE_TILE_SIZE: f32 = 1.5;

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
                    BRICK_TEXTURE_TILE_SIZE * (0.75 + 0.25 * (half_size.z - z) / size.z)
                } else {
                    z + half_size.z
                }
            } else {
                y + half_size.y
            };
            // One texture tile contains four brick rows at a fixed world-space scale.
            [u / BRICK_TEXTURE_TILE_SIZE, v / BRICK_TEXTURE_TILE_SIZE]
        })
        .collect::<Vec<_>>();

    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_1, uvs);
    mesh.with_generated_tangents().unwrap()
}
