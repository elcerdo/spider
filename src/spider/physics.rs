use super::SpiderData;

use bevy::math::{Mat2, Quat, Vec2, Vec3};
use bevy::prelude::{ButtonInput, KeyCode};
use bevy::prelude::{Entity, Query, Res, Time, Transform};
use bevy::prelude::{Gamepad, GamepadAxis, GamepadButton};

use std::f32::consts::PI;

struct VehiclePhysics {
    mass: f32,
    friction: Vec2,
    thrust: f32,
    brake: f32,
    turning_speed: f32,
    target_speed: f32,
    capture_speed: f32,
    dt: f32,
}

impl VehiclePhysics {
    fn from_dt(dt: f32) -> Self {
        Self {
            mass: 100.0,                     // kg
            friction: Vec2::new(1e-2, 5e-2), // 0 <= f < 1
            thrust: 4000.0,                  // m / s^2 / kg ~ N
            brake: 1000.0,                   // m / s^2 / kg ~ N
            turning_speed: 5.0 * PI / 4.0,   // rad / s
            target_speed: 20.0,              // m / s
            capture_speed: 2.0,              // 1 / s
            dt,                              // s
        }
    }
}

impl VehiclePhysics {
    fn compute_next_pos(
        &self,
        pos_prev: Vec2,
        pos_current: Vec2,
        angle_current: f32,
        force: Vec2,
    ) -> Vec2 {
        let half_accel = force / self.mass / 2.0;
        let pp = Mat2::from_angle(angle_current);
        let friction = pp.transpose() * Mat2::from_diagonal(self.friction) * pp;
        (2.0 * Mat2::IDENTITY - friction) * pos_current
            - (1.0 * Mat2::IDENTITY - friction) * pos_prev
            + half_accel * self.dt * self.dt
    }
}

pub fn update_vehicle_physics(
    mut vehicles: Query<(&mut SpiderData, &mut Transform)>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    let physics = VehiclePhysics::from_dt(time.delta_secs());

    for (mut vehicle, mut transform) in &mut vehicles {
        let pos_prev = vehicle.position_previous;
        let pos_current = vehicle.position_current;
        let mut force = Vec2::ZERO;

        {
            if keyboard.pressed(KeyCode::ArrowLeft) {
                vehicle.angle_current += physics.turning_speed * physics.dt;
            }
            if keyboard.pressed(KeyCode::ArrowRight) {
                vehicle.angle_current -= physics.turning_speed * physics.dt;
            }
            let dir_current = Vec2::from_angle(-vehicle.angle_current);
            if keyboard.pressed(KeyCode::ArrowUp) {
                force += physics.thrust * dir_current;
            }
            if keyboard.pressed(KeyCode::ArrowDown) {
                force -= physics.brake * dir_current;
            }
        }

        {
            for (_, gamepad) in &gamepads {
                let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
                let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
                if left_stick_x.abs() > 0.05 {
                    let speed = (Vec2::X + Vec2::Y) * physics.target_speed;
                    vehicle.position_target += speed * left_stick_x * physics.dt;
                }
                if left_stick_y.abs() > 0.05 {
                    let speed = (Vec2::X - Vec2::Y) * physics.target_speed;
                    vehicle.position_target += speed * left_stick_y * physics.dt;
                }
                vehicle.is_target_captured = gamepad.pressed(GamepadButton::East);
            }
        }

        let pos_next = if vehicle.is_target_captured {
            // Moves towards target
            let alpha = physics.capture_speed * physics.dt;
            let alpha = alpha.clamp(0.0, 1.0);
            pos_current * (1.0 - alpha) + vehicle.position_target * alpha
        } else {
            // Integrate Newton second law with anisotropic friction
            physics.compute_next_pos(pos_prev, pos_current, vehicle.angle_current, force)
        };

        vehicle.position_previous = vehicle.position_current;
        vehicle.position_current = pos_next;
        transform.translation = Vec3::new(pos_next.x, 0.0, pos_next.y);
        transform.rotation = Quat::from_axis_angle(Vec3::Y, vehicle.angle_current);
    }
}
