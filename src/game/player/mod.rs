use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Player {
    player_id: String,
}

impl Default for Player {
    fn default() -> Player {
        Self {
            player_id: "todo".to_string(),
        }
    }
}

impl Player {
    pub fn id(&self) -> String {
        self.player_id.clone()
    }
}
