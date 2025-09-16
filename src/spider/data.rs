use bevy::prelude::*;

use std::collections::BTreeMap;

pub enum Controller {
    Gamepad,
    Keyboard,
}

#[derive(Component)]
pub struct SpiderVehicle {
    pub controller: Controller,
    position_initial: Vec2,
    pub position_previous: Vec2,
    pub position_current: Vec2,
    angle_initial: f32,
    pub angle_current: f32,
}

impl SpiderVehicle {
    pub fn new(pos: Vec2, angle: f32, controller: Controller) -> Self {
        Self {
            position_initial: pos,
            position_previous: pos,
            position_current: pos,
            angle_initial: angle,
            angle_current: angle,
            controller,
        }
    }

    pub fn reset(&mut self) {
        self.position_previous = self.position_initial;
        self.position_current = self.position_initial;
        self.angle_current = self.angle_initial;
    }
}

pub struct SpiderLeg {
    pub parent: Entity,
    pub marker: Entity,
    pub entity: Entity,
}

#[derive(Component, Default)]
pub struct SpiderLegs(pub BTreeMap<(String, String), SpiderLeg>);

#[derive(Component)]
pub struct SpiderAnimation {
    pub graph: Handle<AnimationGraph>,
    pub weighted_nodes: Vec<(f32, AnimationNodeIndex)>,
}

#[derive(Component)]
pub struct SpiderTheme {
    pub color_aa: LinearRgba,
    pub color_bb: LinearRgba,
}
