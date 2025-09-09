use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct SpiderData {
    position_initial: Vec2,
    pub position_target: Vec2,
    pub position_previous: Vec2,
    pub position_current: Vec2,
    angle_initial: f32,
    pub angle_current: f32,
    pub is_target_captured: bool,
}

impl SpiderData {
    pub fn from_position_and_angle(pos: Vec2, angle: f32) -> Self {
        Self {
            position_initial: pos,
            position_target: pos,
            position_previous: pos,
            position_current: pos,
            angle_initial: angle,
            angle_current: angle,
            is_target_captured: false,
        }
    }

    pub fn reset(&mut self) {
        self.position_target = self.position_initial;
        self.position_previous = self.position_initial;
        self.position_current = self.position_initial;
        self.angle_current = self.angle_initial;
        self.is_target_captured = false;
    }
}
