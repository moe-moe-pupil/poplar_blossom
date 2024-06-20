pub mod animate;
pub mod camera;
pub mod card;
pub mod deck;
pub mod hand;
pub mod player;
pub mod slot;
pub mod net;
pub mod actions;
use std::f32::consts::PI;

use bevy::{
    pbr::CascadeShadowConfigBuilder,
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
use deck::DeckPlugin;
use hand::HandPlugin;
use slot::SlotPlugin;
use net::NetPlugin;

use crate::{AppState, CardsHandle};

use self::{
    camera::PlayerCameraPlugin,
    card::{Card, CardInfo},
};
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalData>()
            .add_plugins((CardPlugin, HandPlugin, SlotPlugin, DeckPlugin))
            .add_plugins(PlayerCameraPlugin)
            .add_systems(Startup, set_up)
            .add_systems(Update, spawn_cards.run_if(in_state(AppState::LoadingCards)));
    }
}

#[derive(Resource)]
pub struct LocalData {
    player_id: String,
}

impl FromWorld for LocalData {
    fn from_world(world: &mut World) -> Self {
        Self {
            player_id: "todo".to_string(),
        }
    }
}

fn set_up(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            rotation: Quat::from_rotation_x(0.0),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50., 50.)),
        material: materials.add(Color::SILVER),
        transform: Transform::from_rotation(Quat::from_rotation_x(PI / 2.0))
            .with_translation(Vec3::new(0.0, 0.0, -0.1)),
        ..default()
    });
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
                visibility: default(),
                computed_visibiltiy: default(),
            });
        }
        state.set(AppState::Playing);
    }
}
