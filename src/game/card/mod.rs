use bevy::prelude::*;
mod animations;

use animations::Animations;
pub struct CardPlugin;

pub struct ISizeWithMax {
    pub current: isize,
    pub max: isize,
}

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Bundle)]
struct CardBundle {
    pub card: Card,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Component)]
pub struct Card {
    pub animations: Animations,
    pub info: CardInfo,
    pub z: usize,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum CardType {
    Creature,
}

#[derive(Debug)]
pub struct CardStats {
    pub health: isize,
    pub damage: usize,
}

pub struct CardInfo {
    pub card_type: CardType,
    pub stats: CardStats,
}
