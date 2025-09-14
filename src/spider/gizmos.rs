use super::super::ui::UiState;
use super::data::SpiderLegs;
use super::data::SpiderVehicle;
use super::physics::lift;

use bevy::math::NormedVectorSpace;
use bevy::prelude::*;

use super::leg::SPIDER_LEG_LENGTH;

use bevy::color::palettes::css::*;

pub fn display_vehicles(
    ui_state: ResMut<UiState>,
    vehicles: Query<&SpiderVehicle>,
    mut gizmos: Gizmos,
) {
    if !ui_state.display_gizmos {
        return;
    }
    for vehicle in vehicles.iter() {
        gizmos.cross(lift(vehicle.position_target), 5.0, BLUE_VIOLET);
        gizmos.sphere(lift(vehicle.position_current), 2.0, GREEN_YELLOW);
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
