use super::data::SpiderAnimation;

use bevy::prelude::*;

use bevy::scene::SceneInstanceReady;

pub fn populate_animations(
    trigger: Trigger<SceneInstanceReady>,
    animations: Query<&SpiderAnimation>,
    children: Query<&Children>,
    mut commands: Commands,
    mut players: Query<&mut AnimationPlayer>,
) {
    info!("** populate animations **");

    let target = trigger.target();
    if let Ok(animation) = animations.get(target) {
        for child in children.iter_descendants(target) {
            if let Ok(mut player) = players.get_mut(child) {
                // Tell the animation player to start the animation and keep
                // repeating it.
                //
                // If you want to try stopping and switching animations, see the
                // `animated_mesh_control.rs` example.
                // player.play(animation.index_idle).repeat();
                // player.play(animation.weighted_nodes[0].1);

                // Add the animation graph. This only needs to be done once to
                // connect the animation player to the mesh.
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation.graph.clone()));
            }
        }
    }
}

/// Takes the weights that were set in the UI and assigns them to the actual
/// playing animation.
pub fn update_weights(
    // mut query: Query<(&mut AnimationPlayer, &SpiderAnimation)>,
    // mut commands: Commands,
    animations: Query<(&SpiderAnimation, Entity)>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
) {
    // if let Ok(animation) = animations.get(target) {
    for (animation, entity) in animations.iter() {
        for child in children.iter_descendants(entity) {
            if let Ok(mut player) = players.get_mut(child) {
                for (weight, node) in animation.weighted_nodes.iter() {
                    let node = *node;

                    // If the animation happens to be no longer active, restart it.
                    if !player.is_playing_animation(node) {
                        warn!("start {:?}", node);
                        player.play(node).repeat();
                    }

                    // Set the weight.
                    if let Some(active_animation) = player.animation_mut(node) {
                        info!("set weight {}", weight);
                        active_animation.set_weight(*weight);
                    }
                }
            }
        }
    }

    // for (mut player, animation) in query.iter_mut() {
    //     for (weight, node) in animation.weighted_nodes.iter() {
    //         let node = *node;

    //         // If the animation happens to be no longer active, restart it.
    //         if !player.is_playing_animation(node) {
    //             info!("restart {:?}", node);
    //             player.play(node);
    //         }

    //         // Set the weight.
    //         if let Some(active_animation) = player.animation_mut(node) {
    //             info!("set weight {}", weight);
    //             active_animation.set_weight(*weight);
    //         }
    //     }
}
