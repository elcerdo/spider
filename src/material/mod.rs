pub mod parallax_material;
pub mod racing_line_material;
pub mod wavy_material;

use bevy::prelude::*;

pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<racing_line_material::RacingLineMaterial>::default());
        app.add_systems(
            Update,
            (racing_line_material::animate, wavy_material::animate),
        );
    }
}
