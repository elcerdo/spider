use super::data::SpiderAnimation;

use bevy::prelude::*;

use bevy::scene::SceneInstanceReady;

pub fn populate_animations(
    trigger: Trigger<SceneInstanceReady>,
    animations: Query<&SpiderAnimation>,
    children: Query<&Children>,
    players: Query<&AnimationPlayer>,
    mut commands: Commands,
) {
    info!("** populate animations **");
    let target = trigger.target();
    if let Ok(animation) = animations.get(target) {
        for child in children.iter_descendants(target) {
            if players.get(child).is_ok() {
                info!("num_weighted_nodes {}", animation.weighted_nodes.len());
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation.graph.clone()));
            }
        }
    }
}

pub fn update_weights(
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
                        warn!("starting node {:?}", node);
                        player.play(node).repeat();
                    }

                    // Set the weight.
                    if let Some(active_animation) = player.animation_mut(node) {
                        active_animation.set_weight(*weight);
                    }
                }
            }
        }
    }
}
