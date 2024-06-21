use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Player {
    player_id: String,
    health: i32,
}

impl Default for Player {
    fn default() -> Player {
        Self {
            player_id: "todo".to_string(),
            health: Player::DEFAULT_HEALTH,
        }
    }
}

impl Player {
    pub const DEFAULT_HEALTH: i32 = 20;
    pub fn id(&self) -> String {
        self.player_id.clone()
    }
}
