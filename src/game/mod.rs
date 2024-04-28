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
use card::{CardBundle, CardPlugin};

use crate::{AppState, CardsHandle};

use self::{
    camera::PlayerCameraPlugin,
    card::{Card, CardData, CardInfo, CardStats, ISizeWithMax},
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
    card_data: Res<CardData>,
    mut images: ResMut<Assets<Image>>,
) {
    if asset_server.get_recursive_dependency_load_state(&cards.0)
        == Some(bevy::asset::RecursiveDependencyLoadState::Loaded)
    {
        for (_, card_info) in card_infos.iter() {
            let size = Extent3d {
                width: 512,
                height: 512,
                ..default()
            };

            // This is the texture that will be rendered to.
            let mut image = Image {
                texture_descriptor: TextureDescriptor {
                    label: None,
                    size,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Bgra8UnormSrgb,
                    mip_level_count: 1,
                    sample_count: 1,
                    usage: TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_DST
                        | TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                },
                ..default()
            };

            // fill image.data with zeroes
            image.resize(size);
            let image_handle = images.add(image);
            let texture_camera = commands
                .spawn(Camera2dBundle {
                    camera: Camera {
                        // render before the "main pass" camera
                        order: -1,
                        target: RenderTarget::Image(image_handle.clone()),
                        ..default()
                    },
                    camera_2d: Camera2d {},
                    ..default()
                })
                .id();

            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        card_info.desc.clone(),
                        TextStyle {
                            font_size: 64.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.5, 64., 10.),
                    ..default()
                },
                TargetCamera(texture_camera),
            ));

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("card_base.png"),
                    // transform: Transform::from_scale(Vec3::splat(1.)),
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: 512., y: 512. }),
                        ..Default::default()
                    },
                    ..default()
                },
                TargetCamera(texture_camera),
            ));

            commands.spawn(CardBundle {
                transform: Transform::from_xyz(0.5, 0.0, 0.0),
                global_transform: default(),
                card: Card::from(card_info.clone()),
                image_handle,
            });
        }
        state.set(AppState::Playing);
    }
}
