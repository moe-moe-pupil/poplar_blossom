use std::{f32::consts::PI, ops::Range, time::Duration};

use bevy::{prelude::Timer, time::TimerMode};

use crate::game::{animate::*, slot::Slot};

use super::Card;

pub struct CardAnimations {
    pub select: AnimateRange,
    pub rotate_x: AnimateRange,
    pub rotate_y: AnimateRange,
}

impl Default for CardAnimations {
    fn default() -> Self {
        let rotate_animate: AnimateRange = AnimateRange::new(
            Duration::from_secs_f32(0.2),
            Ease::Linear,
            0.0..PI / 16.0,
            false,
            Some(1.0),
        );
        Self {
            select: AnimateRange::new(
                Duration::from_secs_f32(0.2),
                Ease::Linear,
                0.0..Card::FLOATING_HEIGHT,
                false,
                None,
            ),
            rotate_x: rotate_animate.clone(),
            rotate_y: rotate_animate.clone(),
        }
    }
}
