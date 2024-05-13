use std::{f32::consts::PI, ops::Range, time::Duration};

use bevy::{prelude::Timer, time::TimerMode};

use crate::game::animate::*;

pub struct Animations {
    pub select: AnimateRange,
    pub rotate_x: AnimateRange,
    pub rotate_y: AnimateRange,
}

impl Default for Animations {
    fn default() -> Self {
        Self {
            select: AnimateRange::new(
                Duration::from_secs_f32(0.2),
                Ease::Linear,
                0.0..4.0,
                false,
                None,
            ),
            rotate_x: AnimateRange::new(
                Duration::from_secs_f32(0.2),
                Ease::Linear,
                0.0..PI / 16.0,
                false,
                Some(1.0),
            ),
            rotate_y: AnimateRange::new(
                Duration::from_secs_f32(0.2),
                Ease::Linear,
                0.0..PI / 16.0,
                false,
                Some(1.0),
            ),
        }
    }
}
