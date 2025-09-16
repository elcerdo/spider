use super::data::SpiderGun;
use super::data::SpiderVehicle;

use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;

pub fn populate_gun(
    trigger: Trigger<SceneInstanceReady>,
    children: Query<&Children>,
    names: Query<&Name>,
    mut commands: Commands,
) {
    info!("** populate gun **");

    let target = trigger.target();

    let entity = {
        let re = regex::Regex::new(r"^bone_gun_forward$").unwrap();
        let mut entity = None;
        for child in children.iter_descendants(target) {
            if let Ok(name) = names.get(child) {
                if re.captures(name).is_some() {
                    warn!("!!!!! {}", name);
                    entity = Some(child);
                    break;
                }
            }
        }
        entity.unwrap()
    };

    commands.entity(target).insert(SpiderGun {
        entity,
        is_shooting: false,
    });
}

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
