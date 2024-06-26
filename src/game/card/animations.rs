use std::{f32::consts::PI, ops::Range, time::Duration};

use bevy::{math::Quat, prelude::Timer, time::TimerMode};

use crate::game::{animate::*, camera::PlayerCamera, slot::Slot};

use super::Card;

pub struct CardAnimations {
    pub select: AnimateRange,
    pub rotate_x: AnimateRange,
    pub rotate_y: AnimateRange,
    pub rotate_z: AnimateRange,
}

impl Default for CardAnimations {
    fn default() -> Self {
        // TODO: fix the over-rotation
        let rotate_animate: AnimateRange = AnimateRange::new(
            Duration::from_secs_f32(0.2),
            Ease::Linear,
            -0.01..PI / 16.0 + 0.01,
            false,
            Some(1.0),
            0.0,
        );
        Self {
            select: AnimateRange::new(
                Duration::from_secs_f32(0.2),
                Ease::Linear,
                0.0..Card::FLOATING_HEIGHT,
                false,
                None,
                0.0,
            ),
            rotate_x: rotate_animate
                .clone()
                .set_default_value(Quat::from_rotation_x(PlayerCamera::CAMERA_ROTATION_X).x)
                .to_owned(),
            rotate_y: rotate_animate.clone(),
            rotate_z: rotate_animate.clone().set_range(0.0..PI / 2.0).to_owned(),
        }
    }
}
