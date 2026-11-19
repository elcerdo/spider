//! Bullet sweeps select the nearest enemy hitbox or high wall, ignoring low walls.
//! Queries use the XZ plane, independently of the animated muzzle's height.

use avian3d::prelude::*;
use bevy::prelude::*;

use super::data::{SpiderBullet, SpiderVehicle};
use super::gun::{SPIDER_BULLET_DIAM, SPIDER_BULLET_HALF_LENGTH};
use super::physics::{CollisionLayer, lift};

pub const SPIDER_BULLET_RADIUS: f32 = 2.0;

#[derive(Component)]
pub struct VehicleHitbox(pub Entity);

pub fn resolve_bullet_collisions(
    mut commands: Commands,
    mut vehicles: Query<&mut SpiderVehicle>,
    mut bullets: Query<(Entity, &mut SpiderBullet, &Position, &Rotation)>,
    hitboxes: Query<&VehicleHitbox>,
    spatial_query: SpatialQuery,
) {
    let shape = Collider::cuboid(
        SPIDER_BULLET_DIAM,
        SPIDER_BULLET_HALF_LENGTH * 2.0,
        SPIDER_BULLET_DIAM,
    );
    let filter = SpatialQueryFilter::from_mask([CollisionLayer::Hitbox, CollisionLayer::HighWall]);

    for (entity, mut bullet, position, rotation) in &mut bullets {
        let origin = lift(bullet.position_previous.xz());
        let distance = position.0.xz().distance(bullet.position_previous.xz());
        let direction = Dir3::new(bullet.direction).expect("Bullet direction must be normalized");
        let hit = spatial_query.cast_shape_predicate(
            &shape,
            origin.into(),
            rotation.0,
            direction,
            &ShapeCastConfig::from_max_distance(distance),
            &filter,
            &|entity| {
                !hitboxes.get(entity).is_ok_and(|hitbox| {
                    vehicles
                        .get(hitbox.0)
                        .expect("Hitbox must belong to a vehicle")
                        .controller
                        == bullet.controller
                })
            },
        );

        if let Some(hit) = hit {
            if let Ok(hitbox) = hitboxes.get(hit.entity) {
                vehicles
                    .get_mut(hitbox.0)
                    .expect("Hitbox must belong to a vehicle")
                    .num_hits += 1;
            }
            commands.entity(entity).despawn();
        } else {
            bullet.position_previous = position.0.into();
        }
    }
}
