use super::data::SpiderAnimation;

use bevy::prelude::*;

use bevy::scene::SceneInstanceReady;

pub fn set_color(
    trigger: Trigger<SceneInstanceReady>,
    animations: Query<&SpiderAnimation>,
    children: Query<&Children>,
    mut materials_: Query<&mut MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("** set color **");
    let target = trigger.target();
    if let Ok(animation) = animations.get(target) {
        let material = materials.add(StandardMaterial {
            base_color: animation.color.into(),
            perceptual_roughness: 0.2,
            ..default()
        });
        for child in children.iter_descendants(target) {
            if let Ok(mut material_) = materials_.get_mut(child) {
                let material__ = materials.get(material_.id()).unwrap();
                let color__ = material__.base_color.to_linear();
                let dist__ = (color__.red - 0.0).abs() + (color__.blue - 0.16).abs();
                if dist__ < 0.05 {
                    *material_ = MeshMaterial3d(material.clone());
                }
            }
        }
    }
}

pub fn play_idle(
    trigger: Trigger<SceneInstanceReady>,
    animations: Query<&SpiderAnimation>,
    children: Query<&Children>,
    mut commands: Commands,
    mut players: Query<&mut AnimationPlayer>,
) {
    info!("** playing animation **");

    // The entity we spawned in `setup_mesh_and_animation` is the trigger's target.
    // Start by finding the AnimationToPlay component we added to that entity.
    let target = trigger.target();
    if let Ok(animation) = animations.get(target) {
        // The SceneRoot component will have spawned the scene as a hierarchy
        // of entities parented to our entity. Since the asset contained a skinned
        // mesh and animations, it will also have spawned an animation player
        // component. Search our entity's descendants to find the animation player.
        for child in children.iter_descendants(target) {
            if let Ok(mut player) = players.get_mut(child) {
                // Tell the animation player to start the animation and keep
                // repeating it.
                //
                // If you want to try stopping and switching animations, see the
                // `animated_mesh_control.rs` example.
                player.play(animation.index).repeat();

                // Add the animation graph. This only needs to be done once to
                // connect the animation player to the mesh.
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation.graph.clone()));
            }
        }
    }
}
