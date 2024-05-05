use bevy::{ecs::entity, prelude::*, window::PrimaryWindow};
use meshtext::{MeshGenerator, MeshText, TextSection};
use std::cmp::{max, min};
use std::f32::consts::{PI, TAU};
mod animations;

use animations::Animations;
use bevy_rapier3d::{geometry::Collider, pipeline::QueryFilter, plugin::RapierContext};
use serde::{Deserialize, Serialize};

use super::camera::PlayerCamera;
pub struct CardPlugin;

#[derive(Serialize, Deserialize, Debug)]
pub struct ISizeWithMax {
    pub current: isize,
    pub max: isize,
}

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedCard>()
            .init_resource::<HoverPoint>()
            .init_resource::<CardData>()
            .add_systems(PostUpdate, on_spawn_card)
            .add_systems(Update, (select_card, move_cards).chain());
    }
}

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub collider: Collider,
}

#[derive(Component)]
pub struct Card {
    pub animations: Animations,
    pub info: CardInfo,
    pub z: usize,
    pub player_id: usize,
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

    pub fn is_player_controlled(&self) -> bool {
        return if self.player_id == 0 { true } else { false };
    }
}

#[derive(Default, PartialEq, Eq, Copy, Clone, Resource)]
pub enum SelectedCard {
    Some(Entity),
    #[default]
    None,
}

impl SelectedCard {
    fn is_selected(self, entity: Entity) -> bool {
        match self {
            SelectedCard::Some(e) => e == entity,
            SelectedCard::None => false,
        }
    }
}

#[derive(Default, Resource)]
pub enum HoverPoint {
    Some(Vec3),
    #[default]
    None,
}

fn move_cards(
    time: Res<Time>,
    selected: Res<SelectedCard>,
    hover_point: Res<HoverPoint>,
    mut cards: Query<(Entity, &mut Card, &mut Transform)>,
) {
    for (entity, mut card, mut transform) in &mut cards {
        let mut z_offset = 0.0;
        if selected.is_selected(entity) {
            z_offset += card.animations.select.tick(time.delta());
            if let HoverPoint::Some(hover_point) = *hover_point {
                let delta_translation = (hover_point - transform.translation).xy().clamp(
                    -Vec2::new(PI / 10.0, PI / 10.0),
                    Vec2::new(PI / 10.0, PI / 10.0),
                );
                transform.translation.x = (hover_point.x + transform.translation.x) / 2.0;
                transform.translation.y = (hover_point.y + transform.translation.y) / 2.0;
                transform.rotation.x = (transform.rotation.x - delta_translation.y) / 2.0;
                transform.rotation.y = (transform.rotation.y + delta_translation.x) / 2.0;
            }
        } else {
            z_offset += card.animations.deselect.tick(time.delta());
        }

        transform.translation.z = z_offset;
    }
}

pub fn select_card(
    mut commands: Commands,
    context: Res<RapierContext>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut selected_card: ResMut<SelectedCard>,
    mut hover_point: ResMut<HoverPoint>,
    cameras: Query<(&Camera, &Transform), With<PlayerCamera>>,
    mut cards: Query<&mut Card>,
) {
    let window = windows.single();
    if let Some(cursor) = window.cursor_position() {
        let (camera, camera_transform) = cameras.single();

        let view = camera_transform.compute_matrix();

        let Rect {
            min: viewport_min,
            max: viewport_max,
        } = camera.logical_viewport_rect().unwrap();
        let screen_size = camera.logical_target_size().unwrap();
        let viewport_size = viewport_max - viewport_min;
        let adj_cursor_pos = cursor - Vec2::new(viewport_min.x, screen_size.y - viewport_max.y);
        let projection = camera.projection_matrix();
        let far_ndc = projection.project_point3(Vec3::NEG_Z).z;
        let near_ndc = projection.project_point3(Vec3::Z).z;
        let mut cursor_ndc = (adj_cursor_pos / viewport_size) * 2.0 - Vec2::ONE;
        cursor_ndc.y *= -1.0;
        let ndc_to_world: Mat4 = view * projection.inverse();
        let near = ndc_to_world.project_point3(cursor_ndc.extend(near_ndc));
        let far = ndc_to_world.project_point3(cursor_ndc.extend(far_ndc));
        let direction = far - near;
        let denom = Vec3::Z.dot(direction);
        if denom.abs() > 0.0001 {
            let t = (Vec3::ZERO - near).dot(Vec3::Z) / denom;
            if t >= 0.0 {
                *hover_point = HoverPoint::Some(near + direction * t);
            } else {
                *hover_point = HoverPoint::None;
            }
        } else {
            *hover_point = HoverPoint::None;
        }

        if mouse.just_pressed(MouseButton::Left) {
            let result = context.cast_ray(near, direction, 50.0, true, QueryFilter::new());

            if let Some((entity, _toi)) = result {
                if cards.get(entity).unwrap().is_player_controlled() {
                    let mut card = cards.get_mut(entity).unwrap();
                    // unslot from tile
                    card.animations.select.reset();
                    *selected_card = SelectedCard::Some(entity);
                }
            }
        }
    }

    if mouse.just_released(MouseButton::Left) {
        if let SelectedCard::Some(entity) = *selected_card {
            let mut card = cards.get_mut(entity).unwrap();
            card.animations.deselect.reset();
            *selected_card = SelectedCard::None;
        }
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
    portrait_mesh: Handle<Mesh>,
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
            portrait_mesh: meshes.add(Rectangle::new(Card::ART_ASPECT * 0.65, 0.65)),
            card_base_material: materials.add(card_base_material),
        }
    }
}

impl CardData {}

fn on_spawn_card(
    mut commands: Commands,
    card_data: Res<CardData>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cards: Query<(Entity, &Card), Added<Card>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, card) in &cards {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn(SpatialBundle {
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(PbrBundle {
                        material: card_data.card_base_material.clone(),
                        mesh: card_data.mesh.clone(),
                        ..default()
                    });

                    parent.spawn(PbrBundle {
                        mesh: card_data.portrait_mesh.clone(),
                        material: materials.add(StandardMaterial {
                            base_color_texture: Some(
                                asset_server.load(card.info.name.clone() + ".png"),
                            ),
                            unlit: true,
                            alpha_mode: AlphaMode::Blend,
                            ..default()
                        }),
                        transform: Transform::from_xyz(0.0, -0.08, 0.01),
                        ..default()
                    });
                    let font_data = include_bytes!("../../../assets/sans.ttf");
                    let mut generator = MeshGenerator::new(font_data);
                    let transform = Mat4::from_scale(Vec3::new(0.1, 0.1, 0.01)).to_cols_array();
                    let text_mesh: MeshText = generator
                        .generate_section(&card.info.name, false, Some(&transform))
                        .unwrap();

                    let vertices = text_mesh.vertices;
                    let positions: Vec<[f32; 3]> =
                        vertices.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
                    let uvs = vec![[0f32, 0f32]; positions.len()];

                    let mut mesh = Mesh::new(
                        bevy::render::render_resource::PrimitiveTopology::TriangleList,
                        default(),
                    );
                    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                    mesh.compute_flat_normals();

                    parent
                        // use this bundle to change the rotation pivot to the center
                        .spawn(PbrBundle {
                            mesh: meshes.add(mesh),
                            material: materials.add(StandardMaterial {
                                unlit: true,
                                // alpha_mode: AlphaMode::Blend,
                                base_color: Color::BLACK,
                                ..default()
                            }),
                            // transform mesh so that it is in the center
                            transform: Transform::from_xyz(-0.33, 0.35, 0.03),
                            ..Default::default()
                        });
                });
        });
    }
}

impl From<CardInfo> for Card {
    fn from(card_info: CardInfo) -> Self {
        Self {
            info: card_info,
            player_id: 0,
            animations: default(),
            z: default(),
        }
    }
}
