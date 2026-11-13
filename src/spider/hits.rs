use bevy::math::NormedVectorSpace;
use bevy::math::Vec2;
use bevy::prelude::*;

use kd_tree::{KdPoint, KdTree};

use crate::obstacles::{WallHigh, WallLow};

use super::data::Controller;
use super::data::SpiderBullet;
use super::data::SpiderVehicle;

use super::gun::SPIDER_BULLET_HALF_LENGTH;

pub const SPIDER_BULLET_RADIUS: f32 = 2.0; // m
pub const SPIDER_WALL_RADIUS: f32 = 3.5; // m

#[derive(Default)]
enum BBoxPrimitive {
    #[default]
    None,
    Vehicle {
        controller: Controller,
    },
    Bullet {
        entity: Entity,
        controller: Controller,
        flip: Vec2,
    },
    WallLow {
        entity: Entity,
        brick_wall_size: Vec2,
    },
    WallHigh {
        entity: Entity,
        brick_wall_size: Vec2,
    },
}

#[derive(Default)]
struct BBox {
    low: Vec2,
    high: Vec2,
    primitive: BBoxPrimitive,
}

impl BBox {
    fn valid(&self) -> bool {
        match &self.primitive {
            BBoxPrimitive::Bullet { entity, flip, .. } => {
                assert!(entity != &Entity::PLACEHOLDER);
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
            BBoxPrimitive::WallLow { entity, .. } | BBoxPrimitive::WallHigh { entity, .. } => {
                assert!(entity != &Entity::PLACEHOLDER);
                if self.low.x > self.high.x {
                    return false;
                }
                if self.low.y > self.high.y {
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

    fn from_wall(
        entity: Entity,
        is_high: bool,
        brick_wall_size: Vec2,
        transform: &Transform,
    ) -> Self {
        let primitive = match is_high {
            true => BBoxPrimitive::WallHigh {
                entity,
                brick_wall_size,
            },
            false => BBoxPrimitive::WallLow {
                entity,
                brick_wall_size,
            },
        };
        let center = transform.translation.xz();
        let world_half_extents = brick_wall_size * transform.scale.xz().abs() / 2.0;
        let ret = Self {
            low: center - world_half_extents,
            high: center + world_half_extents,
            primitive,
        };
        assert!(ret.valid());
        ret
    }
}

impl KdPoint for BBox {
    type Scalar = f32;
    type Dim = typenum::U4; // 4 dimensional tree.
    fn at(&self, kk: usize) -> f32 {
        assert!(self.valid());
        match kk {
            0 => self.low.x,
            1 => self.low.y,
            2 => self.high.x,
            3 => self.high.y,
            _ => unreachable!(),
        }
    }
}

enum Intersection {
    Hit,
    Contact { normal: Vec2, depth: f32 },
}

fn check_intersection(aa: &BBox, bb: &BBox) -> Option<Intersection> {
    match (&aa.primitive, &bb.primitive) {
        (
            BBoxPrimitive::Bullet {
                controller: aa_controller,
                ..
            },
            BBoxPrimitive::Vehicle {
                controller: bb_controller,
            },
        ) => {
            if aa_controller == bb_controller {
                return None;
            }
            assert!((bb.high - bb.low).norm() < 1e-5);
            let aa_center = (aa.high + aa.low) / 2.0;
            let bb_center = (bb.high + bb.low) / 2.0;
            // FIXME impl proper segment circle intersection
            ((bb_center - aa_center).norm() < SPIDER_BULLET_RADIUS).then_some(Intersection::Hit)
        }
        (BBoxPrimitive::Bullet { flip, .. }, BBoxPrimitive::WallHigh { .. }) => {
            let size = aa.high - aa.low;
            let start = aa.low + *flip * size;
            let end = aa.high - *flip * size;
            let direction = end - start;
            let mut enter: f32 = 0.0;
            let mut exit: f32 = 1.0;
            for axis in 0..2 {
                if direction[axis] == 0.0 {
                    if start[axis] < bb.low[axis] || start[axis] > bb.high[axis] {
                        return None;
                    }
                } else {
                    let first = (bb.low[axis] - start[axis]) / direction[axis];
                    let last = (bb.high[axis] - start[axis]) / direction[axis];
                    enter = enter.max(first.min(last));
                    exit = exit.min(first.max(last));
                    if enter > exit {
                        return None;
                    }
                }
            }
            Some(Intersection::Hit)
        }
        (
            BBoxPrimitive::Vehicle { .. },
            BBoxPrimitive::WallLow { .. } | BBoxPrimitive::WallHigh { .. },
        ) => {
            let position = aa.low;
            let delta = position - position.clamp(bb.low, bb.high);
            let distance = delta.length();
            if distance >= SPIDER_WALL_RADIUS {
                return None;
            }
            if distance > 0.0 {
                return Some(Intersection::Contact {
                    normal: delta / distance,
                    depth: SPIDER_WALL_RADIUS - distance,
                });
            }

            // A center inside the wall must leave through its nearest face.
            let faces = [
                (-Vec2::X, position.x - bb.low.x),
                (Vec2::X, bb.high.x - position.x),
                (-Vec2::Y, position.y - bb.low.y),
                (Vec2::Y, bb.high.y - position.y),
            ];
            let (normal, depth) = faces
                .into_iter()
                .min_by(|aa, bb| aa.1.total_cmp(&bb.1))
                .unwrap();
            Some(Intersection::Contact {
                normal,
                depth: SPIDER_WALL_RADIUS + depth,
            })
        }
        (BBoxPrimitive::Bullet { .. }, BBoxPrimitive::WallLow { .. }) => None,
        _ => unreachable!(),
    }
}

pub fn resolve_wall_collisions(
    low_walls: Query<(Entity, &Transform, &WallLow), Without<SpiderVehicle>>,
    high_walls: Query<(Entity, &Transform, &WallHigh), Without<SpiderVehicle>>,
    mut vehicles: Query<(&mut SpiderVehicle, &mut Transform)>,
) {
    let wall_bboxes: Vec<_> = low_walls
        .iter()
        .map(|(entity, transform, wall)| {
            BBox::from_wall(entity, false, wall.brick_wall_size, transform)
        })
        .chain(high_walls.iter().map(|(entity, transform, wall)| {
            BBox::from_wall(entity, true, wall.brick_wall_size, transform)
        }))
        .collect();
    if wall_bboxes.is_empty() {
        return;
    }
    let wall_bboxes = KdTree::build_by_ordered_float(wall_bboxes);

    for (mut vehicle, mut transform) in vehicles.iter_mut() {
        let mut position = vehicle.position_current;
        let mut velocity = position - vehicle.position_previous;
        let mut vehicle_bbox =
            BBox::from_vehicle(vehicle.controller.clone(), &super::physics::lift(position));
        let wall_canditates = wall_bboxes.within(&[
            [
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
                position.x - SPIDER_WALL_RADIUS,
                position.y - SPIDER_WALL_RADIUS,
            ],
            [
                position.x + SPIDER_WALL_RADIUS,
                position.y + SPIDER_WALL_RADIUS,
                f32::INFINITY,
                f32::INFINITY,
            ],
        ]);
        for wall_bbox in wall_canditates {
            if let Some(Intersection::Contact { normal, depth }) =
                check_intersection(&vehicle_bbox, wall_bbox)
            {
                position += normal * depth;
                vehicle_bbox.low = position;
                vehicle_bbox.high = position;
                velocity -= normal * velocity.dot(normal).min(0.0);
                vehicle.position_current = position;
                vehicle.position_previous = position - velocity;
                transform.translation.x = position.x;
                transform.translation.z = position.y;
            }
        }
    }
}

pub fn resolve_bullet_collisions(
    mut commands: Commands,
    mut vehicles: Query<(&mut SpiderVehicle, &Transform)>,
    bullets: Query<(&SpiderBullet, &Transform, Entity)>,
    high_walls: Query<(Entity, &Transform, &WallHigh)>,
) {
    let bullet_bboxes: Vec<_> = bullets
        .iter()
        .map(|(bullet, transform, entity)| {
            BBox::from_bullet(
                bullet.controller.clone(),
                entity,
                &transform.translation,
                &bullet.direction,
            )
        })
        .collect();
    let bullet_bboxes = KdTree::build_by_ordered_float(bullet_bboxes);
    if bullet_bboxes.is_empty() {
        return;
    }

    let mut blocked_bullets = std::collections::HashSet::new();
    for (entity, transform, wall) in high_walls.iter() {
        let wall_bbox = BBox::from_wall(entity, true, wall.brick_wall_size, transform);
        let bullet_candidates = bullet_bboxes.within(&[
            [
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
                wall_bbox.low.x,
                wall_bbox.low.y,
            ],
            [
                wall_bbox.high.x,
                wall_bbox.high.y,
                f32::INFINITY,
                f32::INFINITY,
            ],
        ]);
        for bullet_bbox in bullet_candidates {
            let BBoxPrimitive::Bullet {
                entity: bullet_entity,
                ..
            } = &bullet_bbox.primitive
            else {
                unreachable!()
            };
            if blocked_bullets.contains(bullet_entity) {
                continue;
            }
            if check_intersection(bullet_bbox, &wall_bbox).is_some() {
                blocked_bullets.insert(*bullet_entity);
            }
        }
    }

    for (mut vehicle, transform) in vehicles.iter_mut() {
        let vehicle_bbox = &BBox::from_vehicle(vehicle.controller.clone(), &transform.translation);
        let bullet_canditates = bullet_bboxes.within(&[
            [
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
                vehicle_bbox.low.x - SPIDER_BULLET_RADIUS,
                vehicle_bbox.low.y - SPIDER_BULLET_RADIUS,
            ],
            [
                vehicle_bbox.high.x + SPIDER_BULLET_RADIUS,
                vehicle_bbox.high.y + SPIDER_BULLET_RADIUS,
                f32::INFINITY,
                f32::INFINITY,
            ],
        ]);
        for bullet_bbox in bullet_canditates {
            let BBoxPrimitive::Bullet {
                entity: bullet_entity,
                ..
            } = &bullet_bbox.primitive
            else {
                unreachable!()
            };
            if blocked_bullets.contains(bullet_entity) {
                continue;
            }
            if check_intersection(bullet_bbox, &vehicle_bbox).is_some() {
                blocked_bullets.insert(*bullet_entity);
                vehicle.num_hits += 1;
            }
        }
    }

    for bullet_entity in blocked_bullets {
        commands.entity(bullet_entity).despawn();
    }
}
