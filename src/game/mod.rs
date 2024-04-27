pub mod card;
pub mod animate;

use bevy::prelude::*;
use card::CardPlugin;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CardPlugin);
    }
}
