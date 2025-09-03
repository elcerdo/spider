use bevy::math::{ops, Vec2};
use bevy::prelude::Component;

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::time::Duration;

#[derive(Clone, PartialEq)]
pub enum Player {
    One,
    Two,
    Three,
}

impl Display for Player {
    fn fmt(&self, buffer: &mut Formatter) -> Result {
        match self {
            Player::One => write!(buffer, "P1"),
            Player::Two => write!(buffer, "P2"),
            Player::Three => write!(buffer, "P3"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct LapStat {
    top_start: Duration,
    checkpoint_to_tops: HashMap<u8, Duration>,
    top_finish: Duration,
}

impl LapStat {
    fn invalid() -> Self {
        Self {
            top_start: Duration::MAX,
            checkpoint_to_tops: HashMap::new(),
            top_finish: Duration::MAX,
        }
    }

    pub fn from(top: Duration) -> Self {
        Self {
            top_start: top,
            checkpoint_to_tops: HashMap::new(),
            top_finish: Duration::MAX,
        }
    }

    pub fn update(&mut self, top: Duration) {
        self.top_finish = top;
    }

    pub fn completed_lap(&mut self, checkpoint: u8, checkpoint_count: u8, top: Duration) -> bool {
        if checkpoint == 0 {
            if self.top_start == Duration::MAX {
                self.top_start = top;
                false
            } else {
                let mut crossed_all = true;
                for kk in 1..checkpoint_count {
                    crossed_all &= self.checkpoint_to_tops.contains_key(&kk);
                }
                crossed_all
            }
        } else {
            self.checkpoint_to_tops.insert(checkpoint, top);
            false
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.top_start == Duration::MAX {
            return false;
        }
        if self.top_finish == Duration::MAX {
            return false;
        }
        if self.top_start > self.top_finish {
            return false;
        }
        true
    }

    pub fn lap_duration(&self) -> Duration {
        assert!(self.top_start != Duration::MAX);
        assert!(self.top_finish != Duration::MAX);
        assert!(self.top_start <= self.top_finish);
        self.top_finish - self.top_start
    }

    pub fn checkpoint_duration(&self, kk: u8) -> Option<Duration> {
        if self.top_start == Duration::MAX {
            assert!(self.checkpoint_to_tops.is_empty());
            return None;
        }
        assert!(self.top_start != Duration::MAX);
        let checkpoint_top = self.checkpoint_to_tops.get(&kk)?;
        assert!(self.top_start <= *checkpoint_top);
        Some(*checkpoint_top - self.top_start)
    }
}

#[derive(Component, Clone)]
pub struct BoatData {
    pub player: Player,
    position_initial: Vec2,
    pub position_previous: Vec2,
    pub position_current: Vec2,
    angle_initial: f32,
    pub angle_current: f32,
    pub layer: u8,
    pub current_stat: LapStat,
    pub last_stat: LapStat,
    pub best_stat: LapStat,
    pub lap_count: u32,
}

impl BoatData {
    pub fn from_player_position_and_forward(player: Player, pos: Vec2, fwd: Vec2) -> Self {
        let angle = ops::atan2(fwd.x, fwd.y); // FIXME should be the other way around
        Self {
            player,
            position_initial: pos,
            position_previous: pos,
            position_current: pos,
            angle_initial: angle,
            angle_current: angle,
            layer: 0,
            current_stat: LapStat::invalid(),
            last_stat: LapStat::invalid(),
            best_stat: LapStat::invalid(),
            lap_count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.position_previous = self.position_initial;
        self.position_current = self.position_initial;
        self.angle_current = self.angle_initial;
        self.layer = 0;
        self.current_stat = LapStat::invalid();
        self.lap_count = 0;
    }
}
