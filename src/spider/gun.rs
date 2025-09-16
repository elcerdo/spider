use super::data::SpiderGun;
use super::data::SpiderVehicle;

use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;

pub fn populate_gun(
    trigger: Trigger<SceneInstanceReady>,
    children: Query<&Children>,
    names: Query<&Name>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("** populate gun **");

    let target = trigger.target();

    let port_entity = {
        let re = regex::Regex::new(r"^bone_gun_forward$").unwrap();
        let mut entity = None;
        for child in children.iter_descendants(target) {
            if let Ok(name) = names.get(child) {
                if re.captures(name).is_some() {
                    entity = Some(child);
                    break;
                }
            }
        }
        entity.unwrap()
    };

    let bullet_mesh = Cuboid::from_size(vec3(0.2, 1.0, 0.2));
    let bullet_mesh = meshes.add(bullet_mesh);

    let bullet_material = StandardMaterial {
        base_color: YELLOW_200.into(),
        emissive: YELLOW_200.into(),
        ..default()
    };
    let bullet_material = materials.add(bullet_material);

    commands.entity(target).insert(SpiderGun {
        port_entity,
        bullet_mesh,
        bullet_material,
        is_shooting: false,
        last_top: time.elapsed_secs_f64(),
    });
}

pub fn update_guns_00(
    gamepads: Query<&Gamepad>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut guns: Query<(&mut SpiderGun, &SpiderVehicle)>,
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

pub fn update_guns_01(
    time: Res<Time>,
    mut guns: Query<&mut SpiderGun>,
    mut commands: Commands,
    global_transforms: Query<&GlobalTransform>,
) {
    let top = time.elapsed_secs_f64();
    for mut gun in guns.iter_mut() {
        let delta = top - gun.last_top;
        gun.last_top = if gun.is_shooting && delta > 0.15 {
            debug!("pop {delta}");
            let transform = global_transforms.get(gun.port_entity).unwrap();
            let transform = Transform {
                translation: transform.translation(),
                rotation: transform.rotation(),
                scale: Vec3::splat(1.0),
            };
            commands.spawn((
                Mesh3d(gun.bullet_mesh.clone()),
                MeshMaterial3d(gun.bullet_material.clone()),
                transform,
            ));
            top
        } else {
            gun.last_top
        };
    }
}
