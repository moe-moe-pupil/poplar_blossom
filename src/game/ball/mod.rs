use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;

use super::systemsets::PlayingSets;
pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallData>()
            .add_systems(Startup, spawn_ball.in_set(PlayingSets::Main))
            .add_systems(Update, on_spawn_ball.in_set(PlayingSets::Main))
            .add_systems(Update, rotate_to_velocity.in_set(PlayingSets::Main));
    }
}

#[derive(Component)]
pub struct Ball {
    power: u32,
}

impl Ball {
    pub const BALL_DEFAULT_SPEED: f32 = 1.0;
    pub const BALL_DEFAULT_RADIUS: f32 = 0.1;
    pub fn try_deal_damage() {
        //TODO: deal damage
        todo!();
    }
}

pub fn rotate_to_velocity(mut balls: Query<&mut Velocity, With<Ball>>) {
    for mut velocity in balls.iter_mut() {
        velocity.angvel = Vec3::new(velocity.linvel.y, velocity.linvel.x, 0.0)
    }
}

pub fn spawn_ball(mut commands: Commands) {
    commands.spawn(BallBundle {
        ball: Ball { power: 1 },
        transform: Transform::from_xyz(0.0, 0.0, 0.258),
        global_transform: default(),
        collider: Collider::ball(Ball::BALL_DEFAULT_RADIUS),
        visibility: default(),
        computed_visibiltiy: default(),
        rigid_body: RigidBody::Dynamic,
    });
}

pub fn on_spawn_ball(
    mut commands: Commands,
    balls: Query<(Entity, &Ball), Added<Ball>>,
    ball_data: Res<BallData>,
) {
    for (ball_entity, ball) in balls.iter() {
        commands
            .entity(ball_entity)
            .with_children(|parent| {
                parent
                    .spawn(SpatialBundle {
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(PbrBundle {
                            material: ball_data.ball_base_material.clone(),
                            mesh: ball_data.mesh.clone(),
                            ..default()
                        });
                    });
            })
            .insert(Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            })
            .insert(Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Max,
            })
            .insert(LockedAxes::TRANSLATION_LOCKED_Z)
            .insert(AdditionalMassProperties::Mass(1.0))
            // .insert(Ccd::enabled())
            .insert(Velocity::linear(Vec3::new(0.0, 1.0, 0.0)));
    }
}

#[derive(Resource)]
pub struct BallData {
    mesh: Handle<Mesh>,
    ball_base_material: Handle<StandardMaterial>,
}

impl FromWorld for BallData {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let card_base_material = StandardMaterial {
            alpha_mode: AlphaMode::Blend,
            base_color: Color::BLUE,
            ..default()
        };
        Self {
            mesh: meshes.add(Sphere::new(Ball::BALL_DEFAULT_RADIUS)),
            ball_base_material: materials.add(card_base_material),
        }
    }
}

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub collider: Collider,
    pub visibility: Visibility,
    pub computed_visibiltiy: InheritedVisibility,
    pub rigid_body: RigidBody,
}
