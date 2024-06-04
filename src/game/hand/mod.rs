use bevy::{prelude::*, window::PrimaryWindow};
use meshtext::{error::MeshTextError, MeshGenerator, MeshText, TextSection};
use bevy_rapier3d::{geometry::Collider, pipeline::QueryFilter, plugin::RapierContext};
use serde::{Deserialize, Serialize};

use super::camera::PlayerCamera;

pub struct HandPlugin;

impl Plugin for HandPlugin {
    fn build(&self, app: &mut App) {
        // TODO
    }
}
