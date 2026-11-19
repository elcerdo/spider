//! Avian integrates planar vehicle motion and resolves wall contacts at a fixed timestep.
//! Vehicle bodies do not collide with each other; separate sensors define bullet hit areas.

use super::data::{Controller, SpiderVehicle};

use avian3d::prelude::*;
use bevy::prelude::*;

use std::f32::consts::PI;

pub const SPIDER_WALL_RADIUS: f32 = 3.5;
const MASS: f32 = 100.0;
const THRUST: f32 = 2000.0;
const STRAFE_THRUST: f32 = 4000.0;
const BRAKE: f32 = 2000.0;
const TURNING_SPEED: f32 = 3.5 * PI / 4.0;
const DRAG: Vec3 = Vec3::new(0.6, 0.0, 3.0);

#[derive(PhysicsLayer, Default)]
pub(crate) enum CollisionLayer {
    #[default]
    Vehicle,
    Wall,
    HighWall,
    Hitbox,
}

pub fn vehicle_body() -> impl Bundle {
    (
        RigidBody::Dynamic,
        Collider::cylinder(SPIDER_WALL_RADIUS, 1.0),
        CollisionLayers::new(CollisionLayer::Vehicle, CollisionLayer::Wall),
        Mass(MASS),
        NoAutoMass,
        LockedAxes::new()
            .lock_translation_y()
            .lock_rotation_x()
            .lock_rotation_z(),
        ConstantForce::default(),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        SweptCcd::default(),
    )
}

pub fn lift(position: Vec2) -> Vec3 {
    Vec3::new(position.x, 0.0, position.y)
}

pub fn update_vehicles(
    mut vehicles: Query<(
        &SpiderVehicle,
        &Rotation,
        &LinearVelocity,
        &mut AngularVelocity,
        &mut ConstantForce,
    )>,
    time: Res<Time>,
    gamepads: Query<&Gamepad>,
    keyboard: Res<ButtonInput<KeyCode>>,
    camera: Single<&Transform, (With<Camera3d>, Without<SpiderVehicle>)>,
) {
    let apply_deadzone = |v: Vec2| {
        if v.length() < 0.05 { Vec2::ZERO } else { v }
    };
    let camera_right = camera.right();
    let camera_right = Vec2::new(camera_right.x, camera_right.z).normalize();
    let camera_up = Vec2::new(camera_right.y, -camera_right.x);

    for (vehicle, rotation, velocity, mut angular_velocity, mut force) in &mut vehicles {
        let forward = (rotation.0 * Vec3::X).xz();
        let strafe_left = Vec2::new(forward.y, -forward.x);
        let mut thrust = Vec2::ZERO;
        let mut turning = 0.0;

        match vehicle.controller {
            Controller::Keyboard => {
                if keyboard.pressed(KeyCode::ArrowLeft) {
                    turning += TURNING_SPEED;
                }
                if keyboard.pressed(KeyCode::ArrowRight) {
                    turning -= TURNING_SPEED;
                }
                if keyboard.pressed(KeyCode::ArrowUp) {
                    thrust += THRUST * forward;
                }
                if keyboard.pressed(KeyCode::ArrowDown) {
                    thrust -= BRAKE * forward;
                }
                if keyboard.pressed(KeyCode::PageUp) {
                    thrust += STRAFE_THRUST * strafe_left;
                }
                if keyboard.pressed(KeyCode::PageDown) {
                    thrust -= STRAFE_THRUST * strafe_left;
                }
            }
            Controller::Gamepad => {
                let angle = -forward.to_angle();
                for gamepad in &gamepads {
                    let aim = apply_deadzone(gamepad.right_stick());
                    if aim != Vec2::ZERO {
                        let direction = camera_right * aim.x + camera_up * aim.y;
                        let target_angle = -direction.to_angle();
                        let angle_delta = (target_angle - angle + PI).rem_euclid(2.0 * PI) - PI;
                        turning =
                            (angle_delta / time.delta_secs()).clamp(-TURNING_SPEED, TURNING_SPEED);
                    }
                    let movement = apply_deadzone(gamepad.left_stick());
                    thrust += THRUST * (camera_right * movement.x + camera_up * movement.y);
                }
            }
        }

        angular_velocity.0 = Vec3::Y * turning;
        let local_velocity = rotation.0.inverse() * velocity.0;
        let drag = rotation.0 * (DRAG * local_velocity) * MASS;
        force.0 = lift(thrust) - drag;
    }
}
