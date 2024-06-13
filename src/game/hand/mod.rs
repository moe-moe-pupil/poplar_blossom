use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{
    geometry::Collider, parry::transformation::utils::transform, pipeline::QueryFilter,
    plugin::RapierContext,
};
use meshtext::{error::MeshTextError, MeshGenerator, MeshText, TextSection};
use serde::{Deserialize, Serialize};

use super::{
    camera::PlayerCamera,
    card::Card,
    player::{self, Player},
    slot::{Slot, SlotBundle, SlotType},
    LocalData,
};

pub struct HandPlugin;

impl Plugin for HandPlugin {
    fn build(&self, app: &mut App) {
        // TODO
        app.add_systems(Startup, spawn_hand)
            .add_systems(Update, on_spawn_hand)
            .add_systems(PostUpdate, recalc_hand_transform);
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
        if hand_player.id() == local_data.player_id {
            let rotation_diff = PI / 8.0 / hand.slots.len() as f32;
            let translation_diff = 3.5 / hand.slots.len() as f32;
            let mut rotation = ((hand.slots.len() / 2) as f32) * rotation_diff;
            let mut translation = 0.0 - ((hand.slots.len() / 2) as f32) * translation_diff;
            for slot_entity in hand.slots.iter() {
                if let Ok(mut transform) = transforms.get_mut(*slot_entity) {
                    transform.rotation = Quat::from_rotation_z(rotation);
                    transform.rotate_x(PlayerCamera::CAMERA_ROTATION_X);
                    transform.translation = Vec3::new(
                        translation as f32,
                        -3.0 - (translation.abs() / rotation.abs().sin()
                            - translation.abs() / rotation.abs().tan()),
                        Card::FLOATING_HEIGHT,
                    );
                    translation += translation_diff;
                    rotation -= rotation_diff;
                }
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

fn spawn_hand(mut commands: Commands) {
    commands.spawn(HandBundle {
        hand: Hand { slots: vec![] },
        player: Player::default(),
    });
    for x in -1..=2 {
        // let mut transfrom: Transform =
        //     Transform::from_translation(Vec3::new(x as f32 / 2.0, -1.5, Card::FLOATING_HEIGHT))
        //         .with_rotation(Quat::from_rotation_x(PlayerCamera::CAMERA_ROTATION_X));
        // transfrom.rotate_around(
        //     Vec3::new(0.0, -5.0, 0.0),
        //     Quat::from_rotation_z(-x as f32 * PI / 3.0 / 18.0),
        // );
        commands.spawn(SlotBundle {
            slot: Slot::new(SlotType::Hand, None),
            // transform: transfrom,
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
