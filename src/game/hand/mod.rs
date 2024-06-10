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
    for x in -3..=3 {
        let mut transfrom: Transform = Transform::from_translation(Vec3::new(
            x as f32 / 2.0,
            -1.5,
            Card::FLOATING_HEIGHT,
        ))
        .with_rotation(Quat::from_rotation_x(PlayerCamera::CAMERA_ROTATION_X));
        transfrom.rotate_around(Vec3::new(0.0, -5.0, 0.0),Quat::from_rotation_z(-x as f32 * PI / 3.0 / 18.0));
        commands.spawn(SlotBundle {
            slot: Slot::new(SlotType::Hand, None),
            transform: transfrom,
            ..default()
        });
    }
}

fn put_card_into_hand() {}

fn on_spawn_hand() {}
