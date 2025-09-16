use super::data::SpiderAnimation;
use super::data::SpiderVehicle;

use bevy::prelude::*;

use bevy::scene::SceneInstanceReady;

pub fn populate_animations(
    trigger: Trigger<SceneInstanceReady>,
    animations: Query<&SpiderAnimation>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
    mut commands: Commands,
) {
    info!("** populate animations **");
    let target = trigger.target();
    if let Ok(animation) = animations.get(target) {
        for child in children.iter_descendants(target) {
            if let Ok(mut player) = players.get_mut(child) {
                let mut try_start = |node: AnimationNodeIndex| {
                    if !player.is_playing_animation(node) {
                        info!("starting node {:?}", node);
                        player.play(node).repeat();
                    }
                };
                try_start(animation.node_idle);
                try_start(animation.node_shoot);
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation.graph.clone()));
            }
        }
    }
}

pub fn update_animations(
    animations: Query<(&SpiderAnimation, &SpiderVehicle, Entity)>,
    children: Query<&Children>,
    gamepads: Query<&Gamepad>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut AnimationPlayer>,
) {
    use super::data::Controller;
    for (animation, vehicle, entity) in animations.iter() {
        let is_shooting = match vehicle.controller {
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

        for child in children.iter_descendants(entity) {
            if let Ok(mut player) = players.get_mut(child) {
                // let mut set_playback = |node: AnimationNodeIndex, aa: bool| match aa {
                //     true => {
                //         if !player.is_playing_animation(node) {
                //             info!("starting node {:?}", node);
                //             player.play(node).repeat();
                //         }
                //         assert!(player.is_playing_animation(node));
                //     }
                //     false => {
                //         if player.is_playing_animation(node) {
                //             info!("starting node {:?}", node);
                //             player.stop(node);
                //         }
                //         assert!(!player.is_playing_animation(node));
                //     }
                // };
                // set_playback(animation.node_shoot, is_shooting);

                let mut try_set_weight = |node: AnimationNodeIndex, weight: f32| {
                    assert!(player.is_playing_animation(node));
                    if let Some(active_animation) = player.animation_mut(node) {
                        active_animation.set_weight(weight);
                    } else {
                        warn!("failed to set weight");
                    }
                };
                let ww = if is_shooting { 1.0 } else { 0.0 };
                try_set_weight(animation.node_idle, 1.0);
                try_set_weight(animation.node_shoot, ww);
            }
        }
    }
}
