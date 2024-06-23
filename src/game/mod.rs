pub mod actions;
pub mod animate;
pub mod ball;
pub mod battlefield;
pub mod camera;
pub mod card;
pub mod deck;
pub mod hand;
pub mod menu;
pub mod net;
pub mod player;
pub mod slot;
pub mod systemsets;

pub mod utils;
use ball::BallPlugin;
use bevy::prelude::*;
use bevy_gltf_blueprints::GltfBlueprintsSet;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_rapier3d::geometry::{
    ActiveCollisionTypes, ActiveEvents, Collider, ComputedColliderShape,
};
use bevy_rapier3d::prelude::*;
use card::CardPlugin;
use deck::DeckPlugin;
use hand::HandPlugin;
use menu::MenuPlugin;
use net::NetPlugin;
use slot::SlotPlugin;
use utils::*;

use crate::{AppState, CardsHandle};

use self::{
    camera::PlayerCameraPlugin,
    card::{Card, CardInfo},
};
pub struct GamePlugin;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub enum BlenderCollider {
    Ball(f32),
    Cuboid(Vec3),
    Capsule(Vec3, Vec3, f32),
    #[default]
    Mesh,
}
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalData>()
            .register_type::<BlenderCollider>()
            .add_plugins((
                CardPlugin,
                HandPlugin,
                SlotPlugin,
                DeckPlugin,
                MenuPlugin,
                BallPlugin,
                // BattlefieldPlugin,
                ComponentsFromGltfPlugin::default(),
            ))
            .add_systems(
                Update,
                physics_replace_proxies.after(GltfBlueprintsSet::AfterSpawn),
            )
            .add_plugins(PlayerCameraPlugin)
            .add_systems(Startup, set_up)
            .add_systems(Update, check_loading.run_if(in_state(AppState::Loading)));
    }
}

#[derive(Resource)]
pub struct LocalData {
    player_id: String,
}

#[allow(clippy::type_complexity)]
pub(crate) fn physics_replace_proxies(
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut proxy_colliders: Query<
        (Entity, &BlenderCollider, &Name, &mut Visibility),
        Added<BlenderCollider>,
    >,
    // needed for tri meshes
    children: Query<&Children>,

    mut commands: Commands,
) {
    for proxy_colider in proxy_colliders.iter_mut() {
        let (entity, collider_proxy, name, mut visibility) = proxy_colider;
        // we hide the collider meshes: perhaps they should be removed altogether once processed ?
        if name.ends_with("_collider") || name.ends_with("_sensor") {
            *visibility = Visibility::Hidden;
        }
        let mut rapier_collider: Collider;
        match collider_proxy {
            BlenderCollider::Ball(radius) => {
                info!("generating collider from proxy: ball");
                rapier_collider = Collider::ball(*radius);
                commands.entity(entity)
                    .insert(rapier_collider)
                    .insert(ActiveEvents::COLLISION_EVENTS)  // FIXME: this is just for demo purposes !!!
                    ;
            }
            BlenderCollider::Cuboid(size) => {
                info!("generating collider from proxy: cuboid");
                rapier_collider = Collider::cuboid(size.x, size.y, size.z);
                commands.entity(entity)
                    .insert(rapier_collider)
                    .insert(ActiveEvents::COLLISION_EVENTS)  // FIXME: this is just for demo purposes !!!
                    ;
            }
            BlenderCollider::Capsule(a, b, radius) => {
                info!("generating collider from proxy: capsule");
                rapier_collider = Collider::capsule(*a, *b, *radius);
                commands.entity(entity)
                    .insert(rapier_collider)
                    .insert(ActiveEvents::COLLISION_EVENTS)  // FIXME: this is just for demo purposes !!!
                    ;
            }
            BlenderCollider::Mesh => {
                info!("generating collider from proxy: mesh");
                for (_, collider_mesh) in
                    Mesh::search_in_children(entity, &children, &meshes, &mesh_handles)
                {
                    rapier_collider =
                        Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh)
                            .unwrap();
                    commands
                        .entity(entity)
                        .insert(rapier_collider)
                        // FIXME: this is just for demo purposes !!!
                        .insert(
                            ActiveCollisionTypes::default()
                                | ActiveCollisionTypes::KINEMATIC_STATIC
                                | ActiveCollisionTypes::STATIC_STATIC
                                | ActiveCollisionTypes::DYNAMIC_STATIC,
                        )
                        .insert(ActiveEvents::COLLISION_EVENTS)
                        .insert(RigidBody::Fixed);
                    //  .insert(ActiveEvents::COLLISION_EVENTS)
                    // break;
                    // Collider::convex_hull(points)
                }
            }
        }
    }
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
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>
) {
    rapier_config.gravity = Vec3::new(0.0, 0.0, -9.8);
    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         illuminance: light_consts::lux::OVERCAST_DAY,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 0.0, 1.0),
    //         rotation: Quat::from_rotation_x(0.0),
    //         ..default()
    //     },
    //     // The default cascade config is designed to handle large scenes.
    //     // As this example has a much smaller world, we can tighten the shadow
    //     // bounds for better visual quality.
    //     cascade_shadow_config: CascadeShadowConfigBuilder {
    //         first_cascade_far_bound: 4.0,
    //         maximum_distance: 10.0,
    //         ..default()
    //     }
    //     .into(),
    //     ..default()
    // });
    // // ground plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Plane3d::default().mesh().size(50., 50.)),
    //     material: materials.add(Color::SILVER),
    //     transform: Transform::from_rotation(Quat::from_rotation_x(PI / 2.0))
    //         .with_translation(Vec3::new(0.0, 0.0, -0.1)),
    //     ..default()
    // });
}

fn check_loading(
    asset_server: Res<AssetServer>,
    cards: Res<CardsHandle>,
    mut state: ResMut<NextState<AppState>>,
) {
    if asset_server.get_recursive_dependency_load_state(&cards.0)
        == Some(bevy::asset::RecursiveDependencyLoadState::Loaded)
    {
        // state.set(AppState::MainMenu);
    }
}
