use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::AppState;

use super::{card::CardInfo, hand::Hand, systemsets::PlayingSets};
pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EvtDrawCardFromDeck>()
            .add_systems(OnEnter(AppState::Playing), deck_setup)
            .add_systems(Update, (
                on_deck_spawn_card,
            ).in_set(PlayingSets::Main));
    }
}

#[derive(Component)]
pub struct Deck {
    slots: Vec<CardInfo>,
}

impl Deck {
    fn pop(&mut self) -> Option<CardInfo> {
        self.slots.pop()
    }

    fn new() -> Self {
        Deck { slots: Vec::new() }
    }
}

#[derive(Event)]
pub struct EvtDrawCardFromDeck {
    pub card_infos: Vec<CardInfo>,
}

fn deck_setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    card_infos: Res<Assets<CardInfo>>,
) {
    let mut deck = Deck::new();
    for (_, card_info) in card_infos.iter() {
        deck.slots.push(card_info.clone());
    }
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.5, 1.0)),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(2.1, -2.4, 0.).with_scale(Vec3::new(0.6, 0.6, 0.6)),
            ..default()
        },
        Name::new("Deck"),
        deck,
    ));
}

// press space and spawn a card
fn on_deck_spawn_card(
    input: Res<ButtonInput<KeyCode>>,
    mut evt: EventWriter<EvtDrawCardFromDeck>,
    mut deck: Query<&mut Deck>,
) {
    if input.just_pressed(KeyCode::Space) {
        let mut deck = deck.single_mut();
        let mut card_infos = Vec::new();
        if let Some(card_info) = deck.pop() {
            card_infos.push(card_info);
        }
        evt.send(EvtDrawCardFromDeck {
            card_infos
        });
    }
} 