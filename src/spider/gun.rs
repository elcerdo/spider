use super::data::SpiderGun;
use super::data::SpiderVehicle;

use bevy::prelude::*;

pub fn update_guns(
    mut guns: Query<(&mut SpiderGun, &SpiderVehicle)>,
    gamepads: Query<&Gamepad>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    use super::data::Controller;
    for (mut gun, vehicle) in guns.iter_mut() {
        gun.is_shooting = match vehicle.controller {
            Controller::Keyboard => keyboard.pressed(KeyCode::Space),
            Controller::Gamepad => {
                let mut any_pressed = false;
                for gamepad in gamepads.iter() {
                    let west_button = gamepad.get(GamepadButton::West).unwrap();
                    any_pressed |= west_button > 0.5;
                }
                any_pressed
            }
        };
    }
}
