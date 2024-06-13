use bevy::prelude::*;

use super::hand::Hand;

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Deck {
    slots: Vec<Entity>,
}

impl Deck {
    pub fn try_draw_card_from_deck(&mut self, hand: &mut Hand) -> bool {
        if let Some(slot_entity) = self.slots.pop() {
            hand.try_put_card_into_hand(slot_entity);
            return true;
        }
        false
    }
}
