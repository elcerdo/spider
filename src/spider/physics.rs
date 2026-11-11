use super::data::SpiderVehicle;

use bevy::math::{Mat2, Quat, Vec2, Vec3};
use bevy::prelude::*;

use std::f32::consts::PI;

struct PhysicsSettings {
    mass: f32,
    friction: Vec2,
    thrust: f32,
    strafe_thrust: f32,
    brake: f32,
    turning_speed: f32,
    dt: f32,
}

impl PhysicsSettings {
    fn from_dt(dt: f32) -> Self {
        Self {
            mass: 100.0,                     // kg
            friction: Vec2::new(1e-2, 5e-2), // 0 <= f < 1
            thrust: 4000.0,                  // m / s^2 / kg ~ N
            strafe_thrust: 8000.0,           // N
            brake: 4000.0,                   // m / s^2 / kg ~ N
            turning_speed: 3.5 * PI / 4.0,   // rad / s
            dt,                              // s
        }
    }
}

impl PhysicsSettings {
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

pub fn lift(aa: Vec2) -> Vec3 {
    Vec3::new(aa.x, 0.0, aa.y)
}

pub fn update_vehicles(
    mut vehicles_and_transforms: Query<(&mut SpiderVehicle, &mut Transform)>,
    time: Res<Time>,
    gamepads: Query<&Gamepad>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    use super::data::Controller;
    let physics = PhysicsSettings::from_dt(time.delta_secs());
    for (mut vehicle, mut transform) in vehicles_and_transforms.iter_mut() {
        // Turn main body
        match vehicle.controller {
            Controller::Keyboard => {
                if keyboard.pressed(KeyCode::ArrowLeft) {
                    vehicle.angle_current += physics.turning_speed * physics.dt;
                }
                if keyboard.pressed(KeyCode::ArrowRight) {
                    vehicle.angle_current -= physics.turning_speed * physics.dt;
                }
            }
            Controller::Gamepad => {
                for gamepad in gamepads.iter() {
                    let right_stick_x = gamepad.get(GamepadAxis::RightStickX).unwrap();
                    if right_stick_x.abs() > 0.05 {
                        vehicle.angle_current -= physics.turning_speed * right_stick_x * physics.dt;
                    }
                }
            }
        };

        // Compute force
        let dir_current = Vec2::from_angle(-vehicle.angle_current);
        let strafe_left = Vec2::new(dir_current.y, -dir_current.x);
        let mut force = Vec2::ZERO;
        match vehicle.controller {
            Controller::Keyboard => {
                if keyboard.pressed(KeyCode::ArrowUp) {
                    force += physics.thrust * dir_current;
                }
                if keyboard.pressed(KeyCode::ArrowDown) {
                    force -= physics.brake * dir_current;
                }
                if keyboard.pressed(KeyCode::PageUp) {
                    force += physics.strafe_thrust * strafe_left;
                }
                if keyboard.pressed(KeyCode::PageDown) {
                    force -= physics.strafe_thrust * strafe_left;
                }
            }
            Controller::Gamepad => {
                for gamepad in gamepads.iter() {
                    let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
                    if left_stick_y > 0.05 {
                        force += physics.thrust * dir_current * left_stick_y;
                    }
                    if left_stick_y < -0.05 {
                        force += physics.brake * dir_current * left_stick_y;
                    }
                    let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
                    if left_stick_x.abs() > 0.05 {
                        force -= physics.strafe_thrust * strafe_left * left_stick_x;
                    }
                }
            }
        };

        // Integrate Newton second law with anisotropic friction
        let pos_next = physics.compute_next_pos(
            vehicle.position_previous,
            vehicle.position_current,
            vehicle.angle_current,
            force,
        );

        // Update state and transform
        vehicle.position_previous = vehicle.position_current;
        vehicle.position_current = pos_next;
        transform.translation = lift(pos_next);
        transform.rotation = Quat::from_axis_angle(Vec3::Y, vehicle.angle_current);
    }
}
