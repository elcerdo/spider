use super::super::ui::UiState;
use super::data::SpiderData;
use super::leg::SpiderAnimation;
use super::physics::lift;

use bevy::math::NormedVectorSpace;
use bevy::prelude::*;

use super::leg::SPIDER_LEG_LENGTH;

use bevy::color::palettes::css::*;

pub fn display_body(ui_state: ResMut<UiState>, vehicles: Query<&SpiderData>, mut gizmos: Gizmos) {
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
    animations: Query<&SpiderAnimation>,
    global_transforms: Query<&GlobalTransform>,
    mut gizmos: Gizmos,
) {
    if !ui_state.display_gizmos {
        return;
    }
    for animation in animations.iter() {
        for leg in animation.legs.values() {
            let transform = global_transforms.get(leg.parent).unwrap();
            let pos = transform.transform_point(Vec3::Y * SPIDER_LEG_LENGTH);
            let pos__ = transform.transform_point(Vec3::ZERO);
            assert!((pos__ - transform.translation()).norm() < 1e-5);
            gizmos.arrow(pos__, pos, WHITE);
        }
    }
}
