use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::{na::distance, prelude::Collider};

use crate::game::card::{Card, CardBundle, CardType, HoverPoint, SelectedCard};

pub struct SlotPlugin;

impl Plugin for SlotPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoveredSlot>()
            .init_resource::<SlotData>()
            .add_systems(Startup, spawn_slots)
            .add_systems(PostUpdate, on_spawn_slot)
            .add_systems(Update, hover_slot.after(crate::game::card::select_card))
            .add_systems(Update, evaluate_slots.after(hover_slot));
    }
}

fn spawn_slots(mut commands: Commands) {
    // for x in -1..2 {
    //     for y in -1..2 {
    //         commands.spawn(SlotBundle {
    //             slot: Slot(None),
    //             ..default()
    //         });
    //     }
    // }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Slot(Option<Entity>);

impl Default for Slot {
    fn default() -> Self {
        Self(None)
    }
}

impl Slot {
    pub const SIZE: Vec2 = Vec2::from_array([3.0, 3.0]);
    pub const OFFSET: Vec2 = Vec2::from_array([-0.05, -0.05]);
    pub const SLOT_ASPECT_RATIO: f32 = 50.0 / 60.0;
    pub const SLOT_SIZE: f32 = 1.2;
    pub const SPAWN_OFFSET: f32 = 0.95;

    pub fn get_nearest_slot<'a>(
        slots: &'a Query<(Entity, &Slot, &Transform)>,
        translation: Vec2,
    ) -> Option<(Entity, &'a Slot)> {
        let mut return_value: Option<(Entity, &'a Slot)> = None;
        let mut max_distance = 500.0;
        for (slot_entity, slot, transform) in slots.iter() {
            let distance = transform.translation.truncate().distance(translation);
            if distance < max_distance {
                return_value = Some((slot_entity, slot));
                max_distance = distance;
            }
            return_value = None;
        }
        return_value
    }

    pub fn grid_to_translation(grid_location: IVec2) -> Vec3 {
        (grid_location.as_vec2() * (Self::SIZE + Self::OFFSET)).extend(0.0)
    }

    pub fn translation_to_grid(translation: Vec3) -> IVec2 {
        let size = Self::SIZE + Self::OFFSET;
        let sign = translation.truncate().signum();
        let grid = (translation.truncate() + sign * size / 2.0) / size;
        grid.as_ivec2()
    }

    pub fn slot_size() -> Vec2 {
        Slot::SLOT_SIZE * Vec2::new(Slot::SLOT_ASPECT_RATIO, 1.0)
    }

    pub fn has_slot(&self) -> bool {
        match self {
            Slot(None) => false,
            _ => true,
        }
    }

    pub fn try_slotting_card(
        &mut self,
        commands: &mut Commands,
        slot_entity: Entity,
        card_entity: Entity,
        card: &Card,
    ) -> bool {
        match self.0 {
            None => true,
            _ => false,
        }
    }
}

#[derive(Bundle, Default)]
pub struct SlotBundle {
    pub slot: Slot,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibiltiy: InheritedVisibility,
}

#[derive(Resource)]
pub struct SlotData {
    mesh: Handle<Mesh>,
    slot_base_material: Handle<StandardMaterial>,
}

impl FromWorld for SlotData {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let asset_server = world.resource::<AssetServer>();
        Self {
            mesh: meshes.add(Rectangle {
                half_size: Vec2::new(3.0, 3.0) / 2.0,
                ..default()
            }),
            slot_base_material: materials.add(StandardMaterial {
                unlit: true,
                depth_bias: -10.0,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
        }
    }
}

#[derive(Default, Deref, DerefMut, Resource)]
pub struct SlotGrid(HashMap<IVec2, Entity>);

fn on_spawn_slot(
    mut commands: Commands,
    slot_data: Res<SlotData>,
    mut slots: Query<(Entity, &mut Slot, &mut Transform), Added<Slot>>,
) {
    for (entity, mut slot, mut transform) in &mut slots {
        commands.entity(entity).with_children(|parent| {
            parent.spawn(PbrBundle {
                material: slot_data.slot_base_material.clone(),
                mesh: slot_data.mesh.clone(),
                visibility: Visibility::Hidden,
                ..default()
            });
        });
        // commands.entity(entity).insert(Slot(None));
    }
}

pub fn enemy_slot_spawner(
    mut commands: Commands,
    mut timer: Local<Option<Timer>>,
    mut grid_size: Local<UVec2>,
    time: Res<Time>,
) {
    if *grid_size == UVec2::new(0, 0) {
        *grid_size = UVec2::new(3, 3);
    }
    let timer = timer.get_or_insert(Timer::new(
        Duration::from_secs_f32(60.0),
        TimerMode::Repeating,
    ));
    if timer.tick(time.delta()).just_finished() {}
}

#[derive(Default, Resource)]
pub struct HoveredSlot(pub Option<Entity>);

pub fn hover_slot(
    hover_point: Res<HoverPoint>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut hovered_slot: ResMut<HoveredSlot>,
    selected_card: Res<SelectedCard>,
    slots: Query<(Entity, &Slot, &Transform)>,
    mut visibilities: Query<&mut Visibility>,
) {
    if let Some(slot_entity) = hovered_slot.0 {
        if let Ok((slot_entity, slot, transfrom)) = slots.get(slot_entity) {
            if let Some(slotted_in_entity) = slot.0 {
                let mut visibility = visibilities.get_mut(slotted_in_entity).unwrap();
                *visibility = Visibility::Hidden;
            }
        }
    }
    for (slot_entity, slot, transfrom) in slots.iter() {
        let mut visibility = visibilities.get_mut(slot_entity).unwrap();
        *visibility = if slot.0.is_some() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if let SelectedCard::Some(_) = *selected_card {
        if let HoverPoint::Some(point) = *hover_point {
            let location = Slot::translation_to_grid(point);
            let nearest_slot = Slot::get_nearest_slot(&slots, point.truncate());
            if let Some((slot_entity, slot)) = nearest_slot {
                hovered_slot.0 = Some(slot_entity);
                let mut visibility = visibilities.get_mut(slot_entity).unwrap();
                *visibility = Visibility::Visible;
            } else {
                hovered_slot.0 = None;
            }
        } else {
            hovered_slot.0 = None;
        }
    }
}

fn evaluate_slots(
    mut commands: Commands,
    time: Res<Time>,
    mut slots: Query<(&mut Slot, &Transform)>,
) {
    for (mut slot, transform) in &mut slots {
        //TODO
    }
}
