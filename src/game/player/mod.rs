use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(HealthBarMap(HashMap::new()))
            .add_event::<EvtBeHurt>()
            .add_systems(Startup, setup)
            .add_systems(Update, (
                update_health_bar,
                decrease_health,
                test_health,
            ));

    }
}

#[derive(Component)]
pub struct Player {
    player_id: String,
    health: i32,
}

impl Default for Player {
    fn default() -> Player {
        Self {
            player_id: "todo".to_string(),
            health: Player::DEFAULT_HEALTH,
        }
    }
}

impl Player {
    pub const DEFAULT_HEALTH: i32 = 20;
    pub fn id(&self) -> String {
        self.player_id.clone()
    }
}

// player map to health bar
#[derive(Resource)]
pub struct HealthBarMap(pub HashMap<Entity, Entity>);

#[derive(Component)]
struct HealthBarMain;

#[derive(Component)]
struct HealthBar;

fn setup(
    mut commands: Commands,
    mut health_bar_map: ResMut<HealthBarMap>,
) {
    let health_bar_id = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(5.0),
                left: Val::Percent(25.),
                top: Val::Percent(25.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::ALICE_BLUE.into(),
            ..default()
        },
        HealthBarMain,
    )).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(80.0),
                top: Val::Percent(10.),
                left: Val::Percent(0.),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        }).insert(HealthBar);
    }).insert(Name::new("PlayerHealthBar")).id();

    let player_id = commands.spawn((
        Player::default(),
        Name::new("Player"),
    )).id();

    health_bar_map.0.insert(player_id, health_bar_id);
}

fn update_health_bar(
    mut cmds: Commands,
    players: Query<(Entity, &Player)>,
    mut query: Query<(Entity, &Children), With<HealthBarMain>>,
    mut query2: Query<&mut Style, (With<HealthBar>, Without<HealthBarMain>)>,
    health_bar_map: ResMut<HealthBarMap>,
) {
    for (eid, player) in players.iter() {
        if let Some(health_bar_id) = health_bar_map.0.get(&eid) {
            if let Ok((health_bar_style_e, childs)) = query.get_mut(*health_bar_id) {
                for &child in childs.iter() {
                    println!("child: {:?}", child);
                    if let Ok(mut health_bar_style) = query2.get_mut(child) {
                        let health = player.health;
                        let cur_width = health_bar_style.width.clone();
                        let target_width = Val::Percent((health as f32 / Player::DEFAULT_HEALTH as f32) * 100.0);
                        health_bar_style.width = target_width;

                    }
                }
            }
        }

    }
}


#[derive(Event)]
pub struct EvtBeHurt {
    pub player_entity: Entity,
    pub damage: i32,
}

fn decrease_health(
    mut players: Query<&mut Player>,
    mut events: EventReader<EvtBeHurt>,
) {
    for evt in events.read() {
        let mut player = players.get_mut(evt.player_entity).unwrap();
        player.health -= evt.damage;
    }
}

fn test_health(
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<EvtBeHurt>,
    players: Query<Entity, With<Player>>,
) {
    if input.just_pressed(KeyCode::KeyH) {
        for player_entity in players.iter() {
            events.send(EvtBeHurt {
                player_entity,
                damage: 3,
            });
        }
    }
}