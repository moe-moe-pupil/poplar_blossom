use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{
    geometry::Collider, parry::transformation::utils::transform, pipeline::QueryFilter,
    plugin::RapierContext,
};
use meshtext::{error::MeshTextError, MeshGenerator, MeshText, TextSection};
use serde::{Deserialize, Serialize};

use crate::AppState;

use super::{
    camera::PlayerCamera, card::Card, player::{self, Player}, slot::{Slot, SlotBundle, SlotType}, systemsets::PlayingSets, LocalData
};

pub struct HandPlugin;

impl Plugin for HandPlugin {
    fn build(&self, app: &mut App) {
        // TODO
        app.add_systems(OnEnter(AppState::Playing), spawn_hand)
            .add_systems(Update, on_spawn_hand.in_set(PlayingSets::Main))
            .add_systems(PostUpdate, (test_spawn_hand, recalc_hand_transform).chain().in_set(PlayingSets::Main));
    }
}

#[derive(Component, Debug)]
pub struct Hand {
    slots: Vec<Entity>,
}

fn recalc_hand_transform(
    local_data: Res<LocalData>,
    hands: Query<(&Hand, &Player)>,
    mut transforms: Query<&mut Transform>,
) {
    for (hand, hand_player) in hands.iter() {
        let hand_radians: f32 = PI / 8.0;
        let hand_translation: f32 = 5.0;
        let rotation_diff = hand_radians / (hand.slots.len() + 1) as f32;
        let translation_diff = hand_translation / (hand.slots.len() + 1) as f32;
        let mut rotation: f32 = hand_radians / 2.0;
        let mut translation: f32 = -hand_translation / 2.0;
        for slot_entity in hand.slots.iter() {
            if let Ok(mut transform) = transforms.get_mut(*slot_entity) {
                translation += translation_diff;
                rotation -= rotation_diff;
                transform.rotation = Quat::from_rotation_z(rotation);
                transform.rotate_x(PlayerCamera::CAMERA_ROTATION_X);
                transform.translation = Vec3::new(
                    translation as f32,
                    -3.0 - if rotation == 0.0 {
                        0.0
                    } else {
                        translation.abs() / rotation.abs().sin()
                            - translation.abs() / rotation.abs().tan()
                    },
                    Card::FLOATING_HEIGHT,
                );
            }
        }
    }
}

impl Hand {
    pub fn try_put_card_into_hand(&mut self, entity: Entity) -> bool {
        self.slots.push(entity);
        true
    }
}
#[derive(Bundle)]
struct HandBundle {
    hand: Hand,
    player: Player,
}

fn test_spawn_hand(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyA) {
        commands.spawn(SlotBundle {
            slot: Slot::new(SlotType::Hand, None),
            ..default()
        });
    }
}

fn spawn_hand(mut commands: Commands) {
    commands.spawn(HandBundle {
        hand: Hand { slots: vec![] },
        player: Player::default(),
    });
    for x in 0..1 {
        commands.spawn(SlotBundle {
            slot: Slot::new(SlotType::Hand, None),
            ..default()
        });
    }
}

fn on_spawn_hand(
    mut commands: Commands,
    local_data: Res<LocalData>,
    slots: Query<(Entity, &Slot, &Player), Added<Slot>>,
    mut hands: Query<(&mut Hand, &Player)>,
) {
    for (mut hand, hand_player) in hands.iter_mut() {
        for (slot_entity, slot, slot_player) in slots.iter() {
            if hand_player.id() == slot_player.id() {
                match slot.get_type() {
                    SlotType::Hand => {
                        hand.slots.push(slot_entity);
                    }
                    _ => {}
                }
            }
        }
    }
}
