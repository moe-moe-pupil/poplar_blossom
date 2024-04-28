use std::f32::consts::PI;

use bevy::{ecs::entity, prelude::*};
mod animations;

use animations::Animations;
use serde::{Deserialize, Serialize};
pub struct CardPlugin;

#[derive(Serialize, Deserialize, Debug)]
pub struct ISizeWithMax {
    pub current: isize,
    pub max: isize,
}

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CardData>()
            .add_systems(PostUpdate, on_spawn_card);
    }
}

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub image_handle: Handle<Image>,
}

#[derive(Component)]
pub struct Card {
    pub animations: Animations,
    pub info: CardInfo,
    pub z: usize,
}

impl Card {
    pub const ASPECT_RATIO: f32 = 50.0 / 60.0;
    pub const ART_WIDTH: f32 = 167.0;
    pub const ART_HEIGHT: f32 = 166.0;
    pub const ART_ASPECT: f32 = Self::ART_WIDTH / Self::ART_HEIGHT;
    pub const SPAWN_OFFSET: f32 = 1.0;

    pub fn card_type(&self) -> CardType {
        self.info.card_type
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CardType {
    Creature,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct CardStats {
    pub toughness: isize,
    pub power: isize,
    #[serde(skip)]
    pub toughness_max: isize,
}

#[derive(Serialize, Deserialize, Debug, TypePath, Asset, Clone)]
pub struct CardInfo {
    pub name: String,
    pub desc: String,
    pub card_type: CardType,
    #[serde(flatten)]
    pub stats: CardStats,
}

#[derive(Resource)]
pub struct CardData {
    mesh: Handle<Mesh>,
    card_base_material: Handle<StandardMaterial>,
}

impl FromWorld for CardData {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let asset_server = world.resource::<AssetServer>();
        let card_base_material = StandardMaterial {
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            base_color_texture: Some(asset_server.load("card_base.png")),
            ..default()
        };
        Self {
            mesh: meshes.add(Rectangle::new(Card::ASPECT_RATIO, 1.0)),
            card_base_material: materials.add(card_base_material),
        }
    }
}

impl CardData {}

fn on_spawn_card(
    mut commands: Commands,
    card_data: Res<CardData>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cards: Query<(Entity, &Card, &Handle<Image>), Added<Card>>,
) {
    for (entity, card, image) in &cards {
        commands.entity(entity).with_children(|parent| {
            parent.spawn(PbrBundle {
                material: card_data.card_base_material.clone(),
                mesh: card_data.mesh.clone(),
                ..default()
            });

            parent.spawn(PbrBundle {
                mesh: card_data.mesh.clone(),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(image.clone()),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 0.1),
                ..default()
            });
        });
    }
}

impl From<CardInfo> for Card {
    fn from(card_info: CardInfo) -> Self {
        Self {
            info: card_info,
            animations: default(),
            z: default(),
        }
    }
}
