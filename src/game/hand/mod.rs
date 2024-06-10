use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{geometry::Collider, pipeline::QueryFilter, plugin::RapierContext};
use meshtext::{error::MeshTextError, MeshGenerator, MeshText, TextSection};
use serde::{Deserialize, Serialize};

use super::{
    camera::PlayerCamera,
    card::Card,
    slot::{Slot, SlotBundle, SlotType},
};

pub struct HandPlugin;

impl Plugin for HandPlugin {
    fn build(&self, app: &mut App) {
        // TODO
        app.add_systems(Startup, spawn_hand);
    }
}

#[derive(Component)]
pub struct Hand {
    slots: Vec<Entity>,
}

#[derive(Bundle)]
struct HandBundle {
    Hand: Hand,
}

fn spawn_hand(mut commands: Commands) {
    commands.spawn(HandBundle {
        Hand: Hand { slots: vec![] },
    });
    for x in -3..3 {
        commands.spawn(SlotBundle {
            slot: Slot::new(SlotType::Hand, None),
            transform: Transform::from_xyz(x as f32, -1.0, Card::FLOATING_HEIGHT)
                .with_rotation(Quat::from_rotation_x(PlayerCamera::CAMERA_ROTATION_X)),
            ..default()
        });
    }
}

fn put_card_into_hand() {}

fn on_spawn_hand() {}
