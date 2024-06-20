use bevy::prelude::*;

use crate::AppState;

#[derive(Debug, SystemSet, Hash, PartialEq, Eq, Clone, Copy)]
pub enum PlayingSets {
    Main,
}

pub struct SystemSetsPlugin;

impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Startup, (
                PlayingSets::Main.run_if(in_state(AppState::Playing)),
            ))
            .configure_sets(Update, (
                PlayingSets::Main.run_if(in_state(AppState::Playing)),
            ))
            .configure_sets(PostUpdate, (
                PlayingSets::Main.run_if(in_state(AppState::Playing)),
            ));
    }
}