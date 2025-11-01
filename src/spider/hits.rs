use bevy::math::NormedVectorSpace;
use bevy::math::Vec2;
use bevy::prelude::*;

use kd_tree::{KdPoint, KdTree};

use super::data::SpiderBullet;
use super::data::SpiderVehicle;

use super::gun::SPIDER_BULLET_HALF_LENGTH;

pub const SPIDER_BODY_RADIUS: f32 = 2.0;

#[derive(Default)]
enum BBoxPrimitive {
    #[default]
    None,
    Bullet {
        flip: Vec2,
    },
    Vehicle {
        radius: f32,
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
            BBoxPrimitive::Bullet { flip } => {
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
            BBoxPrimitive::Vehicle { radius } => {
                if self.low.x != self.high.x {
                    return false;
                }
                if self.low.y != self.high.y {
                    return false;
                }
                if radius <= &0.0 {
                    return false;
                }
            }
            BBoxPrimitive::None => {
                return false;
            }
        };
        true
    }

    fn from_bullet(position: &Vec3, direction: &Vec3) -> Self {
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
            primitive: BBoxPrimitive::Bullet { flip },
        };
        assert!(ret.valid());
        ret
    }

    fn from_vehicle(position: &Vec3) -> Self {
        let ret = Self {
            low: position.xz(),
            high: position.xz(),
            primitive: BBoxPrimitive::Vehicle {
                radius: SPIDER_BODY_RADIUS,
            },
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
            1 => self.low.x,
            2 => self.high.y,
            3 => self.high.y,
            _ => unreachable!(),
        }
    }
}

fn check_intersection(aa: &BBox, bb: &BBox) -> bool {
    let BBoxPrimitive::Bullet { flip: _aa_flip } = aa.primitive else {
        unreachable!()
    };
    let aa_center = (aa.high + aa.low) / 2.0;
    let BBoxPrimitive::Vehicle { radius: bb_radius } = bb.primitive else {
        unreachable!()
    };
    assert!((bb.high - bb.low).norm() < 1e-5);
    let bb_center = (bb.high + bb.low) / 2.0;
    (bb_center - aa_center).norm() < bb_radius
}

pub fn detect_hits(
    mut vehicles: Query<(&mut SpiderVehicle, &Transform)>,
    bullets: Query<(&SpiderBullet, &Transform)>,
) {
    let mut bullet_bboxes = vec![];
    for (bullet, transform) in bullets.iter() {
        bullet_bboxes.push(BBox::from_bullet(&transform.translation, &bullet.direction));
    }
    let bullets_kdtree = KdTree::build_by_ordered_float(bullet_bboxes);
    // assert!(!bullets_kdtree.is_empty());

    for (mut vehicle, transform) in vehicles.iter_mut() {
        let vehicle_bbox = &BBox::from_vehicle(&transform.translation);
        let bullet_bboxes = bullets_kdtree.within_radius(vehicle_bbox, 2.0 * SPIDER_BODY_RADIUS);
        let bullet_bboxes: Vec<&BBox> = bullet_bboxes
            .into_iter()
            .filter(|bullet_bbox| check_intersection(bullet_bbox, vehicle_bbox))
            .collect();
        vehicle.num_hits += bullet_bboxes.len();
    }
}
