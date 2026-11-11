use bevy::math::NormedVectorSpace;
use bevy::math::Vec2;
use bevy::prelude::*;

use kd_tree::{KdPoint, KdTree};

use super::data::Controller;
use super::data::SpiderBullet;
use super::data::SpiderVehicle;

use super::gun::SPIDER_BULLET_HALF_LENGTH;

pub const SPIDER_HIT_RADIUS: f32 = 2.0; // m
pub const SPIDER_BODY_RADIUS: f32 = 3.5; // m

#[derive(Default)]
pub(crate) enum BBoxPrimitive {
    #[default]
    None,
    Bullet {
        controller: Controller,
        entity: Entity,
        flip: Vec2,
    },
    Vehicle {
        controller: Controller,
    },
    WallLow {
        entity: Entity,
        brick_wall_size: Vec3,
    },
}

#[derive(Resource, Default)]
pub(crate) struct StaticPrimitives(pub Vec<BBoxPrimitive>);

#[derive(Default)]
struct BBox {
    low: Vec2,
    high: Vec2,
    primitive: BBoxPrimitive,
}

impl BBox {
    fn valid(&self) -> bool {
        match &self.primitive {
            BBoxPrimitive::Bullet { flip, .. } => {
                if self.low.x > self.high.x {
                    return false;
                }
                if self.low.y > self.high.y {
                    return false;
                }
                if flip.x != 0.0 && flip.x != 1.0 {
                    return false;
                }
                if flip.y != 0.0 && flip.y != 1.0 {
                    return false;
                }
            }
            BBoxPrimitive::Vehicle { .. } => {
                if self.low.x != self.high.x {
                    return false;
                }
                if self.low.y != self.high.y {
                    return false;
                }
            }
            BBoxPrimitive::WallLow { .. } => {
                if self.low.x > self.high.x || self.low.y > self.high.y {
                    return false;
                }
            }
            BBoxPrimitive::None => {
                return false;
            }
        };
        true
    }

    fn from_bullet(
        controller: Controller,
        entity: Entity,
        position: &Vec3,
        direction: &Vec3,
    ) -> Self {
        use std::mem::swap;
        assert!(direction.y.abs() < 1e-5);
        let position = position.xz();
        let direction = direction.xz();
        let mut low = position - direction * SPIDER_BULLET_HALF_LENGTH;
        let mut high = position + direction * SPIDER_BULLET_HALF_LENGTH;
        let mut flip = Vec2::ZERO;
        if low.x > high.x {
            swap(&mut low.x, &mut high.x);
            flip.x = 1.0;
        }
        if low.y > high.y {
            swap(&mut low.y, &mut high.y);
            flip.y = 1.0;
        }
        let ret = Self {
            low,
            high,
            primitive: BBoxPrimitive::Bullet {
                controller,
                entity,
                flip,
            },
        };
        assert!(ret.valid());
        ret
    }

    fn from_vehicle(controller: Controller, position: &Vec3) -> Self {
        let ret = Self {
            low: position.xz(),
            high: position.xz(),
            primitive: BBoxPrimitive::Vehicle { controller },
        };
        assert!(ret.valid());
        ret
    }

    fn from_wall(entity: Entity, brick_wall_size: Vec3, transform: &Transform) -> Self {
        let center = transform.translation.xz();
        let world_half_extents = brick_wall_size.xz() * transform.scale.xz().abs() / 2.0;
        let ret = Self {
            low: center - world_half_extents,
            high: center + world_half_extents,
            primitive: BBoxPrimitive::WallLow {
                entity,
                brick_wall_size,
            },
        };
        assert!(ret.valid());
        ret
    }

    fn body_penetration(&self, position: Vec2) -> Option<(Vec2, f32)> {
        let delta = position - position.clamp(self.low, self.high);
        let distance = delta.length();
        if distance >= SPIDER_BODY_RADIUS {
            return None;
        }
        if distance > 0.0 {
            return Some((delta / distance, SPIDER_BODY_RADIUS - distance));
        }

        // A center inside the wall must leave through its nearest face.
        let faces = [
            (-Vec2::X, position.x - self.low.x),
            (Vec2::X, self.high.x - position.x),
            (-Vec2::Y, position.y - self.low.y),
            (Vec2::Y, self.high.y - position.y),
        ];
        let (normal, depth) = faces
            .into_iter()
            .min_by(|aa, bb| aa.1.total_cmp(&bb.1))
            .unwrap();
        Some((normal, SPIDER_BODY_RADIUS + depth))
    }
}

pub fn resolve_wall_collisions(
    primitives: Res<StaticPrimitives>,
    walls: Query<&Transform, Without<SpiderVehicle>>,
    mut vehicles: Query<(&mut SpiderVehicle, &mut Transform)>,
) {
    let wall_bboxes: Vec<_> = primitives
        .0
        .iter()
        .filter_map(|primitive| {
            let BBoxPrimitive::WallLow {
                entity,
                brick_wall_size,
            } = primitive else {
                return None;
            };
            let transform = walls
                .get(*entity)
                .expect("registered low wall must have a transform");
            Some(BBox::from_wall(*entity, *brick_wall_size, transform))
        })
        .collect();
    if wall_bboxes.is_empty() {
        return;
    }

    for (mut vehicle, mut transform) in vehicles.iter_mut() {
        let mut position = vehicle.position_previous;
        let mut velocity = vehicle.position_current - position;
        // Keep each step shorter than the body radius to avoid crossing thin walls.
        let steps = (velocity.length() / (SPIDER_BODY_RADIUS / 2.0))
            .ceil()
            .max(1.0) as usize;
        let mut step = velocity / steps as f32;
        let mut collided = false;
        for _ in 0..steps {
            position += step;
            for wall in &wall_bboxes {
                if let Some((normal, depth)) = wall.body_penetration(position) {
                    collided = true;
                    position += normal * depth;
                    step -= normal * step.dot(normal).min(0.0);
                    velocity -= normal * velocity.dot(normal).min(0.0);
                }
            }
        }
        if collided {
            vehicle.position_current = position;
            vehicle.position_previous = position - velocity;
            transform.translation.x = position.x;
            transform.translation.z = position.y;
        }
    }
}

impl KdPoint for BBox {
    type Scalar = f32;
    type Dim = typenum::U4; // 4 dimensional tree.
    fn at(&self, kk: usize) -> f32 {
        assert!(self.valid());
        match kk {
            0 => self.low.x,
            1 => self.low.x,
            2 => self.high.y,
            3 => self.high.y,
            _ => unreachable!(),
        }
    }
}

fn check_intersection(aa: &BBox, bb: &BBox) -> bool {
    // FIXME impl proper segement circle intersection
    let BBoxPrimitive::Bullet {
        controller: aa_controller,
        ..
    } = &aa.primitive
    else {
        unreachable!()
    };
    let aa_center = (aa.high + aa.low) / 2.0;
    let BBoxPrimitive::Vehicle {
        controller: bb_controller,
    } = &bb.primitive
    else {
        unreachable!()
    };
    if aa_controller == bb_controller {
        return false;
    }
    assert!((bb.high - bb.low).norm() < 1e-5);
    let bb_center = (bb.high + bb.low) / 2.0;
    (bb_center - aa_center).norm() < SPIDER_HIT_RADIUS
}

pub fn resolve_hits(
    mut commands: Commands,
    mut vehicles: Query<(&mut SpiderVehicle, &Transform)>,
    bullets: Query<(&SpiderBullet, &Transform, Entity)>,
) {
    let mut bullet_bboxes = vec![];
    for (bullet, transform, entity) in bullets.iter() {
        bullet_bboxes.push(BBox::from_bullet(
            bullet.controller.clone(),
            entity,
            &transform.translation,
            &bullet.direction,
        ));
    }
    let bullets_kdtree = KdTree::build_by_ordered_float(bullet_bboxes);
    // assert!(!bullets_kdtree.is_empty());

    for (mut vehicle, transform) in vehicles.iter_mut() {
        let vehicle_bbox = &BBox::from_vehicle(vehicle.controller.clone(), &transform.translation);
        let bullet_bboxes = bullets_kdtree.within_radius(vehicle_bbox, 2.0 * SPIDER_HIT_RADIUS);
        let bullet_bboxes: Vec<&BBox> = bullet_bboxes
            .into_iter()
            .filter(|bullet_bbox| check_intersection(bullet_bbox, vehicle_bbox))
            .collect();
        vehicle.num_hits += bullet_bboxes.len();
        bullet_bboxes.iter().for_each(|bullet_bbox| {
            let BBoxPrimitive::Bullet {
                entity: bullet_entity,
                ..
            } = bullet_bbox.primitive
            else {
                unreachable!()
            };
            commands.entity(bullet_entity).despawn();
        });
    }
}
