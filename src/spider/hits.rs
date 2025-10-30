use bevy::math::Vec3;
use bevy::prelude::*;

use kd_tree::{KdPoint, KdTree};

use super::data::SpiderBullet;
use super::data::SpiderVehicle;

use super::gun::SPIDER_BULLET_HALF_LENGTH;

pub const SPIDER_BODY_RADIUS: f32 = 4.0;

#[derive(Default)]
enum BBoxPrimitive {
    #[default]
    None,
    Bullet {
        flip: Vec3,
    },
    Vehicle {
        radius: f32,
    },
}

#[derive(Default)]
struct BBox {
    low: Vec3,
    high: Vec3,
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
                if self.low.z > self.high.z {
                    return false;
                }
                if flip.x != 0.0 && flip.x != 1.0 {
                    return false;
                }
                if flip.y != 0.0 && flip.y != 1.0 {
                    return false;
                }
                if flip.z != 0.0 && flip.z != 1.0 {
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
                if self.low.z != self.high.z {
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
        let mut low = position - direction * SPIDER_BULLET_HALF_LENGTH;
        let mut high = position + direction * SPIDER_BULLET_HALF_LENGTH;
        let mut flip = Vec3::ZERO;
        if low.x > high.x {
            swap(&mut low.x, &mut high.x);
            flip.x = 1.0;
        }
        if low.y > high.y {
            swap(&mut low.y, &mut high.y);
            flip.y = 1.0;
        }
        if low.z > high.z {
            swap(&mut low.z, &mut high.z);
            flip.z = 1.0;
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
            low: position.clone(),
            high: position.clone(),
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
        let vehicle_bbox = BBox::from_vehicle(&transform.translation);
        // let closest_segment = bullets_kdtree.nearest(&vehicle_bbox).unwrap();
        let foo = bullets_kdtree.within_radius(&vehicle_bbox, SPIDER_BODY_RADIUS);
        vehicle.num_hits += foo.len();
    }
}
