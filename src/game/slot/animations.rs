use std::{f32::consts::PI, ops::Range, time::Duration};

use bevy::{prelude::Timer, time::TimerMode};

use crate::game::{animate::*, slot::Slot};

use super::Card;

pub struct SlotAnimations {
    pub slotted: AnimateRange,
}

impl Default for SlotAnimations {
    fn default() -> Self {
        Self {
            slotted: todo!(),
        }
    }
}
