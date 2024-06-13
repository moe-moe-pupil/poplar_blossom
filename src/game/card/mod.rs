use std::f32::consts::PI;

use bevy::{pbr::NotShadowCaster, prelude::*, transform, window::PrimaryWindow};
use meshtext::{error::MeshTextError, MeshGenerator, MeshText, TextSection};
mod animations;

use animations::CardAnimations;
use bevy_rapier3d::{geometry::Collider, pipeline::QueryFilter, plugin::RapierContext};
use serde::{Deserialize, Serialize};
use std::mem;

use crate::game::slot;

use super::{
    camera::PlayerCamera,
    slot::{HoveredSlot, Slot},
};
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
    pub visibility: Visibility,
    pub computed_visibiltiy: InheritedVisibility,
}

#[derive(Component)]
pub struct Card {
    pub animations: CardAnimations,
    pub info: CardInfo,
    pub player_id: String,
    pub slotted_in_slot: Option<Entity>,
}

impl Card {
    pub const ASPECT_RATIO: f32 = 50.0 / 60.0;
    pub const ART_WIDTH: f32 = 167.0;
    pub const ART_HEIGHT: f32 = 166.0;
    pub const FLOATING_HEIGHT: f32 = 1.1;
    pub const ART_ASPECT: f32 = Self::ART_WIDTH / Self::ART_HEIGHT;
    pub const SPAWN_OFFSET: f32 = 1.0;

    pub fn card_type(&self) -> CardType {
        self.info.card_type
    }

    pub fn is_player_controlled(&self) -> bool {
        return if self.player_id == "todo".to_owned() { true } else { false };
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
    hovered_slot: Res<HoveredSlot>,
    mut cards: Query<(Entity, &mut Card, &mut Transform)>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut transforms: Query<&mut Transform, Without<Card>>,
) {
    for (card_entity, mut card, mut transform) in &mut cards {
        let mut z_offset = 0.0;
        if selected.is_selected(card_entity) {
            if !mouse.just_pressed(MouseButton::Left) {
                if let Some(slot_entity) = hovered_slot.0 {
                    if let Some(target_slot_entity) = card.slotted_in_slot {
                        if slot_entity != target_slot_entity {
                            if let Ok([mut transfrom, mut target_transfrom]) =
                                transforms.get_many_mut([slot_entity, target_slot_entity])
                            {
                                mem::swap::<Transform>(&mut transfrom, &mut target_transfrom);
                            }
                        }
                    }
                }
            }
            z_offset += card.animations.select.tick(time.delta());
            if let HoverPoint::Some(hover_point) = *hover_point {
                let delta_translation = (hover_point - transform.translation).xy();
                transform.translation.x = (hover_point.x + transform.translation.x) / 2.0;
                transform.translation.y = (hover_point.y + transform.translation.y) / 2.0;
                transform.rotation.x = card
                    .animations
                    .rotate_x
                    .tick_f32(delta_translation.y * -0.1);
                transform.rotation.y = card.animations.rotate_y.tick_f32(delta_translation.x * 0.1);
                card.animations.rotate_z.set_default_value(0.0);
            }
        } else {
            if let Some(slot_entity) = card.slotted_in_slot {
                let slot_transform = transforms.get(slot_entity).unwrap();
                transform.translation.x =
                    (transform.translation.x + slot_transform.translation.x) / 2.0;
                transform.translation.y =
                    (transform.translation.y + slot_transform.translation.y) / 2.0;
                card.animations
                    .select
                    .set_range(slot_transform.translation.z..Card::FLOATING_HEIGHT)
                    .set_default_value(slot_transform.translation.z);
                card.animations
                    .rotate_x
                    .set_default_value(slot_transform.rotation.x);
                card.animations
                    .rotate_z
                    .set_default_value(slot_transform.rotation.z);
                if slot_transform.translation.z != 0.0 {
                    card.animations
                        .rotate_z
                        .tick_f32((slot_transform.translation.z - transform.translation.z) / 2.0);
                }
            } else {
                card.animations
                    .select
                    .set_range(0.0..Card::FLOATING_HEIGHT)
                    .set_default_value(0.0);
                card.animations
                    .rotate_x
                    .set_default_value(Quat::from_rotation_x(PlayerCamera::CAMERA_ROTATION_X).x);
                card.animations.rotate_z.set_default_value(0.0);
            }
            z_offset += card
                .animations
                .select
                .reverse_tick(time.delta().mul_f32(2.0));
        }

        transform.rotation.x = card
            .animations
            .rotate_x
            .reverse_tick(time.delta().mul_f32(0.5));
        transform.rotation.y = card
            .animations
            .rotate_y
            .reverse_tick(time.delta().mul_f32(0.5));
        transform.rotation.z = card
            .animations
            .rotate_z
            .reverse_tick(time.delta().mul_f32(0.1));
        transform.translation.z = z_offset;
    }
}

pub fn select_card(
    context: Res<RapierContext>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut selected_card: ResMut<SelectedCard>,
    mut hover_point: ResMut<HoverPoint>,
    hovered_slot: Res<HoveredSlot>,
    mut commands: Commands,
    cameras: Query<(&Camera, &Transform), With<PlayerCamera>>,
    mut cards: Query<(&mut Card, &Transform)>,
    mut slots: Query<&mut Slot>,
    mut transforms: Query<&mut Transform, (Without<Card>, Without<Camera>)>,
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
        if let SelectedCard::Some(card_entity) = selected_card.as_ref() {
            let (_card, transfrom) = cards.get(card_entity.clone()).unwrap();

            let denom = Vec3::Z.dot(direction);
            if denom.abs() > 0.0001 {
                let t = ((Vec3::ZERO - near).dot(Vec3::Z) + transfrom.translation.z) / denom;
                if t >= 0.0 {
                    *hover_point = HoverPoint::Some(near + direction * t);
                } else {
                    *hover_point = HoverPoint::None;
                }
            } else {
                *hover_point = HoverPoint::None;
            }
        }

        if mouse.just_pressed(MouseButton::Left) {
            let result = context.cast_ray(near, direction, 50.0, true, QueryFilter::new());
            if let Some((entity, _toi)) = result {
                let (mut card, _transfrom) = cards.get_mut(entity).unwrap();
                if card.is_player_controlled() {
                    // unslot from tile
                    *selected_card = SelectedCard::Some(entity);
                }
            }
        }
    }

    if mouse.just_released(MouseButton::Left) {
        if let SelectedCard::Some(card_entity) = *selected_card {
            let (mut card, mut _transform) = cards.get_mut(card_entity).unwrap();
            *selected_card = SelectedCard::None;
            if let Some(slot_entity) = hovered_slot.0 {
                if let Ok(mut slot) = slots.get_mut(slot_entity) {
                    if slot.try_slotting_card(&mut commands, slot_entity, card_entity, &card) {
                        if let Some(slot_entity) = card.slotted_in_slot {
                            if let Ok(mut slot) = slots.get_mut(slot_entity) {
                                slot.remove_slotted_entity();
                            }
                        }
                        card.slotted_in_slot = Some(slot_entity);
                    }
                }
            }
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
    pub name_zh: String,
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
    card_font_material: Handle<StandardMaterial>,
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
        let card_font_material = StandardMaterial {
            unlit: true,
            // alpha_mode: AlphaMode::Blend,
            base_color: Color::BLACK,
            ..default()
        };
        Self {
            mesh: meshes.add(Rectangle::new(Card::ASPECT_RATIO, 1.0)),
            portrait_mesh: meshes.add(Rectangle::new(Card::ART_ASPECT * 0.65, 0.65)),
            card_base_material: materials.add(card_base_material),
            card_font_material: materials.add(card_font_material),
        }
    }
}

impl CardData {}

const FONT_DATA: &[u8] = include_bytes!("../../../assets/han_rounded.ttf");
fn generate_text_mesh(text: &str) -> Mesh {
    let mut generator = MeshGenerator::new(&FONT_DATA);
    let transform = Mat4::from_scale(Vec3::new(0.1, 0.1, 0.01)).to_cols_array();
    let text_mesh: MeshText = generator
        .generate_section(text, false, Some(&transform))
        .unwrap();

    let vertices = text_mesh.vertices;
    let positions: Vec<[f32; 3]> = vertices.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
    let uvs = vec![[0f32, 0f32]; positions.len()];

    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.compute_flat_normals();
    mesh
}

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

                    parent
                        .spawn(PbrBundle {
                            mesh: card_data.portrait_mesh.clone(),
                            material: materials.add(StandardMaterial {
                                base_color_texture: Some(
                                    asset_server.load(card.info.name.clone() + ".png"),
                                ),
                                unlit: true,
                                alpha_mode: AlphaMode::Blend,
                                ..default()
                            }),
                            transform: Transform::from_xyz(0.0, -0.08, 0.03),
                            ..default()
                        })
                        .insert(NotShadowCaster);
                    let name_mesh = generate_text_mesh(&card.info.name_zh);
                    let toughness_mesh = generate_text_mesh(&card.info.stats.toughness.to_string());
                    let power_mesh = generate_text_mesh(&card.info.stats.power.to_string());
                    parent
                        // use this bundle to change the rotation pivot to the center
                        .spawn(PbrBundle {
                            mesh: meshes.add(name_mesh),
                            material: card_data.card_font_material.clone(),
                            // transform mesh so that it is in the center
                            transform: Transform::from_xyz(-0.33, 0.35, 0.03),
                            ..Default::default()
                        })
                        .insert(NotShadowCaster);
                    [
                        (power_mesh, 1.0, card.info.stats.power.to_string().len()),
                        (
                            toughness_mesh,
                            -1.0,
                            card.info.stats.toughness.to_string().len(),
                        ),
                    ]
                    .map(|(mesh, dir, len)| {
                        parent
                            // use this bundle to change the rotation pivot to the center
                            .spawn(PbrBundle {
                                mesh: meshes.add(mesh),
                                material: card_data.card_font_material.clone(),
                                // transform mesh so that it is in the center
                                transform: Transform::from_xyz(
                                    -0.4 * dir - len as f32 * 0.04,
                                    -0.45,
                                    0.03,
                                )
                                .with_scale(Vec3::new(2.0, 2.0, 1.0)),
                                ..Default::default()
                            })
                            .insert(NotShadowCaster);
                    });
                });
        });
    }
}

impl From<CardInfo> for Card {
    fn from(card_info: CardInfo) -> Self {
        Self {
            info: card_info,
            player_id: "todo".to_owned(),
            animations: default(),
            slotted_in_slot: default(),
        }
    }
}
