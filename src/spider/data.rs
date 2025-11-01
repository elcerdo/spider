use bevy::prelude::*;

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone)]
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
    pub num_hits: usize,
}

impl SpiderVehicle {
    pub fn new(pos: Vec2, angle: f32, controller: Controller) -> Self {
        Self {
            controller,
            position_initial: pos,
            position_previous: pos,
            position_current: pos,
            angle_initial: angle,
            angle_current: angle,
            num_hits: 0,
        }
    }

    pub fn reset(&mut self) {
        self.position_previous = self.position_initial;
        self.position_current = self.position_initial;
        self.angle_current = self.angle_initial;
        self.num_hits = 0;
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
    pub node_idle: AnimationNodeIndex,
    pub node_shoot: AnimationNodeIndex,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Component)]
pub struct SpiderTheme {
    pub color_aa: LinearRgba,
    pub color_bb: LinearRgba,
}

#[derive(Component)]
pub struct SpiderGun {
    pub port_entity: Entity,
    pub bullet_mesh: Handle<Mesh>,
    pub bullet_material: Handle<StandardMaterial>,
    pub is_shooting: bool,
    pub last_top: f64,
}

#[derive(Component)]
pub struct SpiderBullet {
    pub controller: Controller,
    pub position_initial: Vec3,
    pub direction: Vec3,
}
