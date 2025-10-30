use super::super::ui::UiState;
use super::data::SpiderGun;
use super::data::SpiderLegs;
use super::data::SpiderVehicle;
use super::physics::lift;

use bevy::math::NormedVectorSpace;
use bevy::prelude::*;

#[cfg(feature = "debug_gizmos")]
use bevy::scene::SceneInstanceReady;

use super::leg::SPIDER_LEG_LENGTH;
use super::hits::SPIDER_BODY_RADIUS;

use bevy::color::palettes::css::*;

#[cfg(feature = "debug_gizmos")]
pub fn add_reference_axis(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    let target = trigger.target();
    commands.entity(target).with_children(|parent| {
        let mut gizmo = GizmoAsset::new();
        gizmo.arrow(Vec3::ZERO, Vec3::X * 5.0, RED);
        parent.spawn(Gizmo {
            handle: gizmo_assets.add(gizmo),
            ..default()
        });
        let mut gizmo = GizmoAsset::new();
        gizmo.arrow(Vec3::ZERO, Vec3::Y * 5.0, GREEN);
        parent.spawn(Gizmo {
            handle: gizmo_assets.add(gizmo),
            ..default()
        });
        let mut gizmo = GizmoAsset::new();
        gizmo.arrow(Vec3::ZERO, Vec3::Z * 5.0, BLUE);
        parent.spawn(Gizmo {
            handle: gizmo_assets.add(gizmo),
            ..default()
        });
    });
}

pub fn display_vehicles(
    ui_state: ResMut<UiState>,
    vehicles: Query<&SpiderVehicle>,
    mut gizmos: Gizmos,
) {
    if !ui_state.display_gizmos {
        return;
    }
    for vehicle in vehicles.iter() {
        gizmos.sphere(lift(vehicle.position_current), SPIDER_BODY_RADIUS, GREEN_YELLOW);
    }
}

pub fn display_legs(
    ui_state: ResMut<UiState>,
    all_legs: Query<&SpiderLegs>,
    global_transforms: Query<&GlobalTransform>,
    mut gizmos: Gizmos,
) {
    if !ui_state.display_gizmos {
        return;
    }
    for legs in all_legs.iter() {
        for leg in legs.0.values() {
            let transform = global_transforms.get(leg.parent).unwrap();
            let pos = transform.transform_point(Vec3::Y * SPIDER_LEG_LENGTH);
            let pos__ = transform.transform_point(Vec3::ZERO);
            assert!((pos__ - transform.translation()).norm() < 1e-5);
            gizmos.arrow(pos__, pos, WHITE);
        }
    }
}

pub fn display_guns(
    ui_state: ResMut<UiState>,
    guns: Query<&SpiderGun>,
    global_transforms: Query<&GlobalTransform>,
    mut gizmos: Gizmos,
) {
    if !ui_state.display_gizmos {
        return;
    }
    for gun in guns.iter() {
        let transform = global_transforms.get(gun.port_entity).unwrap();
        let pos = transform.transform_point(Vec3::Y * SPIDER_LEG_LENGTH);
        let pos__ = transform.transform_point(Vec3::ZERO);
        assert!((pos__ - transform.translation()).norm() < 1e-5);
        gizmos.arrow(pos__, pos, RED);
    }
}
