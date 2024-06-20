use std::collections::VecDeque;
use bevy::prelude::*;


#[derive(Clone, Copy)]
pub enum PlayerAction {
    // move card from [0] -> [1]
    MoveCard(Vec3, Vec3),

    // place a card in chess
    PlaceCard(Vec3)
}

pub enum PlayerUni {
    // self
    Player1,
    //enemy
    Player2
}

#[derive(Event)]
pub struct PlayerActionEvt {
    player_uni: PlayerUni,
    action: PlayerAction,
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerActionEvt>()
            .add_systems(Update, (
                listen_actions,
            ));
    }
}

fn listen_actions(
    mut player_event: EventReader<PlayerActionEvt>,
) {
    for evt in player_event.read() {
        match evt.player_uni {
            PlayerUni::Player1 => {
            }
            PlayerUni::Player2 => {
                match evt.action {
                    PlayerAction::MoveCard(st, ed) => {
                        
                    }
                    PlayerAction::PlaceCard(pos) => {
                        
                    }
                }
            }
        }
    }
}