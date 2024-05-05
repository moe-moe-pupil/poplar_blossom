pub mod animate;
pub mod camera;
pub mod card;

use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};
use bevy_rapier3d::geometry::Collider;
use card::{CardBundle, CardPlugin};

use crate::{AppState, CardsHandle};

use self::{
    camera::PlayerCameraPlugin,
    card::{Card, CardInfo},
};
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CardPlugin)
            .add_plugins(PlayerCameraPlugin)
            .add_systems(Update, spawn_cards.run_if(in_state(AppState::LoadingCards)));
    }
}

fn spawn_cards(
    mut commands: Commands,
    cards: Res<CardsHandle>,
    asset_server: Res<AssetServer>,
    card_infos: Res<Assets<CardInfo>>,
    mut state: ResMut<NextState<AppState>>,
    mut images: ResMut<Assets<Image>>,
) {
    if asset_server.get_recursive_dependency_load_state(&cards.0)
        == Some(bevy::asset::RecursiveDependencyLoadState::Loaded)
    {
        for (_, card_info) in card_infos.iter() {
            commands.spawn(CardBundle {
                transform: Transform::from_xyz(0.5, 0.0, 0.1),
                global_transform: default(),
                card: Card::from(card_info.clone()),
                collider: Collider::cuboid(Card::ASPECT_RATIO / 2.0, 1.0 / 2.0, 0.2),
            });
        }
        state.set(AppState::Playing);
    }
}
