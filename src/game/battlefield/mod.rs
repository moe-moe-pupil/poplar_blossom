use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;

pub struct BattlefieldPlugin;

impl Plugin for BattlefieldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BattlefieldData>()
            .add_systems(Startup, spawn_battlefield)
            .add_systems(Update, on_spawn_battlefield);
    }
}

#[derive(Component, Default)]
pub struct Battlefield {}

#[derive(Bundle, Default)]
struct BattlefieldBundle {
    pub battlefield: Battlefield,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub collider: Collider,
    pub visibility: Visibility,
    pub computed_visibiltiy: InheritedVisibility,
}

#[derive(Resource)]
pub struct BattlefieldData {
    mesh: Handle<Mesh>,
    battlefield_base_material: Handle<StandardMaterial>,
}

impl FromWorld for BattlefieldData {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let asset_server = world.resource::<AssetServer>();
        let battilefield_base_material = StandardMaterial {
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            // TODO: add texture for the battlefield
            // base_color_texture: Some(asset_server.load("card_base.png")),
            base_color: Color::YELLOW,
            ..default()
        };
        Self {
            mesh: meshes.add(Rectangle::new(
                Battlefield::ASPECT_RATIO * Battlefield::HEIGHT,
                Battlefield::HEIGHT,
            )),
            battlefield_base_material: materials.add(battilefield_base_material),
        }
    }
}

impl Battlefield {
    pub const ASPECT_RATIO: f32 = 50.0 / 60.0;
    pub const HEIGHT: f32 = 5.0;
    pub fn try_enter_battlefiled(mut commands: Commands) {}
}

fn spawn_battlefield(mut commands: Commands) {
    commands.spawn(BattlefieldBundle {
        battlefield: Battlefield {},
        ..default()
    });
}

fn on_spawn_battlefield(
    mut commands: Commands,
    battlefield_data: Res<BattlefieldData>,
    battlefields: Query<(Entity, &Battlefield), Added<Battlefield>>,
) {
    for (entity, battlefield) in &battlefields {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn(SpatialBundle {
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(PbrBundle {
                        material: battlefield_data.battlefield_base_material.clone(),
                        mesh: battlefield_data.mesh.clone(),
                        ..default()
                    });
                });
        });
    }
}
