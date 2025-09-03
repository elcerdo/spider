use crate::vehicle::data::{BoatData, Player};

use bevy::math::{Mat2, Quat, Vec2, Vec3};
use bevy::prelude::{ButtonInput, KeyCode};
use bevy::prelude::{Entity, Query, Res, Time, Transform};
use bevy::prelude::{Gamepad, GamepadAxis, GamepadButton};

use std::f32::consts::PI;

struct BoatPhysics {
    mass: f32,
    friction: Vec2,
    thrust: f32,
    brake: f32,
    turning_speed: f32,
    force: Vec2,
    dt: f32,
}

impl BoatPhysics {
    fn from_dt(dt: f32) -> Self {
        Self {
            mass: 100.0,                     // kg
            friction: Vec2::new(5e-2, 1e-2), // 0 <= f < 1
            thrust: 1500.0,                  // m / s^2 / kg ~ N
            brake: 800.0,                    // m / s^2 / kg ~ N
            turning_speed: 5.0 * PI / 4.0,   // rad / s
            force: Vec2::ZERO,               // m / s^2 /kg ~ N
            dt,                              // s
        }
    }
}

impl BoatPhysics {
    fn compute_next_pos(&self, pos_prev: Vec2, pos_current: Vec2, angle_current: f32) -> Vec2 {
        let accel = self.force / self.mass / 2.0;
        let pp = Mat2::from_angle(angle_current);
        let friction = pp.transpose() * Mat2::from_diagonal(self.friction) * pp;
        (2.0 * Mat2::IDENTITY - friction) * pos_current
            - (1.0 * Mat2::IDENTITY - friction) * pos_prev
            + accel * self.dt * self.dt
    }
}

pub fn update_vehicle_physics(
    mut boats: Query<(&mut BoatData, &mut Transform)>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    let dt = time.delta_secs();
    for (mut boat, mut transform) in &mut boats {
        let pos_prev = boat.position_previous;
        let pos_current = boat.position_current;
        let mut physics = BoatPhysics::from_dt(dt);
        match boat.player {
            Player::One => {
                if keyboard.pressed(KeyCode::ArrowLeft) {
                    boat.angle_current += physics.turning_speed * dt;
                }
                if keyboard.pressed(KeyCode::ArrowRight) {
                    boat.angle_current -= physics.turning_speed * dt;
                }
                let dir_current = Vec2::from_angle(PI / 2.0 - boat.angle_current);
                if keyboard.pressed(KeyCode::ArrowUp) {
                    physics.force += physics.thrust * dir_current;
                }
                if keyboard.pressed(KeyCode::ArrowDown) {
                    // physics.friction = Vec2::ONE * 0.10;
                    physics.force -= physics.brake * dir_current;
                }
            }
            Player::Three => {
                if keyboard.pressed(KeyCode::KeyA) {
                    boat.angle_current += physics.turning_speed * dt;
                }
                if keyboard.pressed(KeyCode::KeyD) {
                    boat.angle_current -= physics.turning_speed * dt;
                }
                let dir_current = Vec2::from_angle(PI / 2.0 - boat.angle_current);
                if keyboard.pressed(KeyCode::KeyW) {
                    physics.force += physics.thrust * dir_current;
                }
                if keyboard.pressed(KeyCode::KeyS) {
                    // physics.friction = Vec2::ONE * 0.10;
                    physics.force -= physics.brake * dir_current;
                }
            }
            Player::Two => {
                for (_, gamepad) in &gamepads {
                    let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
                    if left_stick_x.abs() > 0.01 {
                        boat.angle_current -= physics.turning_speed * left_stick_x * dt;
                    }
                    let dir_current = Vec2::from_angle(PI / 2.0 - boat.angle_current);
                    if gamepad.pressed(GamepadButton::East) {
                        physics.force += physics.thrust * dir_current;
                    }
                    if gamepad.pressed(GamepadButton::North) {
                        // physics.friction = Vec2::ONE * 0.10;
                        physics.force -= physics.brake * dir_current;
                    }
                }
            }
        };
        let pos_next = physics.compute_next_pos(pos_prev, pos_current, boat.angle_current);
        boat.position_previous = boat.position_current;
        boat.position_current = pos_next;
        transform.translation = Vec3::new(pos_next.x, 0.0, pos_next.y);
        transform.rotation = Quat::from_axis_angle(Vec3::Y, boat.angle_current);
    }
}
