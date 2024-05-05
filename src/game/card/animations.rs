use std::{ops::Range, time::Duration};

use bevy::{prelude::Timer, time::TimerMode};

use crate::game::animate::*;

pub struct Animations {
    pub select: AnimateRange,
    pub deselect: AnimateRange,
    pub rotate: AnimateRange,
    pub attack_in: AnimateRange,
    pub attack_out: AnimateRange,
}

impl Default for Animations {
    fn default() -> Self {
        Self {
            select: AnimateRange::new(Duration::from_secs_f32(0.2), Ease::Linear, 0.0..4.0, false),
            deselect: AnimateRange::new(
                Duration::from_secs_f32(0.2),
                Ease::Linear,
                4.0..0.0,
                false,
            ),
            rotate: AnimateRange::new(
                Duration::from_secs_f32(0.2),
                Ease::InOutCirc,
                1.0..0.0,
                false,
            ),
            attack_in: AnimateRange::new(
                Duration::from_secs_f32(0.2),
                Ease::Linear,
                1.0..1.5,
                false,
            ),
            attack_out: AnimateRange::new(
                Duration::from_secs_f32(0.2),
                Ease::Linear,
                1.5..1.0,
                false,
            ),
        }
    }
}
