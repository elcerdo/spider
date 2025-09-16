use super::data::SpiderTheme;

use bevy::prelude::*;

use bevy::math::NormedVectorSpace;
use bevy::scene::SceneInstanceReady;

pub fn set_theme(
    trigger: Trigger<SceneInstanceReady>,
    animations: Query<&SpiderTheme>,
    children: Query<&Children>,
    mut materials_: Query<&mut MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("** set theme **");
    let target = trigger.target();
    if let Ok(animation) = animations.get(target) {
        let material_aa = materials.add(StandardMaterial {
            base_color: animation.color_aa.into(),
            perceptual_roughness: 0.2,
            ..default()
        });
        let material_bb = materials.add(StandardMaterial {
            base_color: animation.color_bb.into(),
            perceptual_roughness: 0.8,
            ..default()
        });
        for child in children.iter_descendants(target) {
            if let Ok(mut material_) = materials_.get_mut(child) {
                let material__ = materials.get(material_.id()).unwrap();
                let color__ = material__.base_color.to_linear();
                let change_to_aa = (color__.to_vec3() - vec3(0.0, 0.07, 0.16)).norm() < 0.05;
                let change_to_bb = (color__.to_vec3() - vec3(0.80, 0.14, 0.0)).norm() < 0.05;
                if change_to_aa {
                    *material_ = MeshMaterial3d(material_aa.clone());
                    continue;
                }
                if change_to_bb {
                    *material_ = MeshMaterial3d(material_bb.clone());
                    continue;
                }
                info!("!!!!!! {} {}", material_.id(), color__.to_vec3());
            }
        }
    }
}
