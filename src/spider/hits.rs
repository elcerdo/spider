use bevy::math::NormedVectorSpace;
use bevy::math::Vec2;
use bevy::prelude::*;

use kd_tree::{KdPoint, KdTree};

use crate::obstacles::{WallHigh, WallLow};

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
            BBoxPrimitive::WallLow {
                entity,
                brick_wall_size,
            }
            | BBoxPrimitive::WallHigh {
                entity,
                brick_wall_size,
            } => {
                if *entity == Entity::PLACEHOLDER
                    || !brick_wall_size.is_finite()
                    || !brick_wall_size.cmpgt(Vec2::ZERO).all()
                {
                    return false;
                }
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

    fn intersect_vehicle(&self, position: Vec2) -> Option<(Vec2, f32)> {
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

    fn intersects_segment(&self, start: Vec2, end: Vec2) -> bool {
        let direction = end - start;
        let mut enter: f32 = 0.0;
        let mut exit: f32 = 1.0;
        for axis in 0..2 {
            if direction[axis] == 0.0 {
                if start[axis] < self.low[axis] || start[axis] > self.high[axis] {
                    return false;
                }
            } else {
                let first = (self.low[axis] - start[axis]) / direction[axis];
                let last = (self.high[axis] - start[axis]) / direction[axis];
                enter = enter.max(first.min(last));
                exit = exit.min(first.max(last));
                if enter > exit {
                    return false;
                }
            }
        }
        true
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
            BBox::from_wall(entity, false, wall.brick_wall_size.xz(), transform)
        })
        .chain(high_walls.iter().map(|(entity, transform, wall)| {
            BBox::from_wall(entity, true, wall.brick_wall_size.xz(), transform)
        }))
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
                if let Some((normal, depth)) = wall.intersect_vehicle(position) {
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
            1 => self.low.y,
            2 => self.high.x,
            3 => self.high.y,
            _ => unreachable!(),
        }
    }
}

fn check_intersection(aa: &BBox, bb: &BBox) -> bool {
    // FIXME impl proper segement circle intersection
    let BBoxPrimitive::Bullet {
        controller: aa_controller,
        flip,
        ..
    } = &aa.primitive
    else {
        unreachable!()
    };
    match &bb.primitive {
        BBoxPrimitive::Vehicle {
            controller: bb_controller,
        } => {
            if aa_controller == bb_controller {
                return false;
            }
            assert!((bb.high - bb.low).norm() < 1e-5);
            let aa_center = (aa.high + aa.low) / 2.0;
            let bb_center = (bb.high + bb.low) / 2.0;
            (bb_center - aa_center).norm() < SPIDER_HIT_RADIUS
        }
        BBoxPrimitive::WallHigh { .. } => {
            let size = aa.high - aa.low;
            let start = aa.low + *flip * size;
            let end = aa.high - *flip * size;
            bb.intersects_segment(start, end)
        }
        _ => unreachable!(),
    }
}

pub fn resolve_vehicle_hits(
    mut commands: Commands,
    mut vehicles: Query<(&mut SpiderVehicle, &Transform)>,
    bullets: Query<(&SpiderBullet, &Transform, Entity)>,
    low_walls: Query<(Entity, &Transform, &WallLow)>,
    high_walls: Query<(Entity, &Transform, &WallHigh)>,
) {
    let mut bboxes = vec![];
    for (bullet, transform, entity) in bullets.iter() {
        bboxes.push(BBox::from_bullet(
            bullet.controller.clone(),
            entity,
            &transform.translation,
            &bullet.direction,
        ));
    }
    bboxes.extend(low_walls.iter().map(|(entity, transform, wall)| {
        BBox::from_wall(entity, false, wall.brick_wall_size.xz(), transform)
    }));
    bboxes.extend(high_walls.iter().map(|(entity, transform, wall)| {
        BBox::from_wall(entity, true, wall.brick_wall_size.xz(), transform)
    }));
    let kdtree = KdTree::build_by_ordered_float(bboxes);
    let mut blocked_bullets = Vec::new();
    for bullet_bbox in kdtree.iter() {
        let BBoxPrimitive::Bullet { entity, .. } = &bullet_bbox.primitive else {
            continue;
        };
        // Overlap requires wall.low <= bullet.high and wall.high >= bullet.low.
        let candidates = kdtree.within(&[
            [
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
                bullet_bbox.low.x,
                bullet_bbox.low.y,
            ],
            [
                bullet_bbox.high.x,
                bullet_bbox.high.y,
                f32::INFINITY,
                f32::INFINITY,
            ],
        ]);
        let (_, bullet_transform, _) = bullets
            .get(*entity)
            .expect("indexed bullet must have a transform");
        let blocked = candidates.into_iter().any(|wall_bbox| {
            let BBoxPrimitive::WallHigh {
                entity: wall_entity,
                ..
            } = &wall_bbox.primitive
            else {
                return false;
            };
            let (_, transform, wall) = high_walls
                .get(*wall_entity)
                .expect("registered high wall must have a transform");
            let center_y = transform.translation.y;
            let half_height = wall.brick_wall_size.y * transform.scale.y.abs() / 2.0;
            (bullet_transform.translation.y - center_y).abs() <= half_height
                && check_intersection(bullet_bbox, wall_bbox)
        });
        if blocked {
            blocked_bullets.push(*entity);
            commands.entity(*entity).despawn();
        }
    }

    for (mut vehicle, transform) in vehicles.iter_mut() {
        let vehicle_bbox = &BBox::from_vehicle(vehicle.controller.clone(), &transform.translation);
        let bullet_bboxes = kdtree.within_radius(vehicle_bbox, 2.0 * SPIDER_HIT_RADIUS);
        let bullet_bboxes: Vec<&BBox> = bullet_bboxes
            .into_iter()
            .filter(|bullet_bbox| {
                let BBoxPrimitive::Bullet { entity, .. } = &bullet_bbox.primitive else {
                    return false;
                };
                !blocked_bullets.contains(entity) && check_intersection(bullet_bbox, vehicle_bbox)
            })
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
