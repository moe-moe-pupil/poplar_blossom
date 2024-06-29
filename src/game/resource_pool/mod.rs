use bevy::prelude::*;

use super::player::Player;

pub struct ResourcePoolPlugin;

impl Plugin for ResourcePoolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_resource_pool)
            .add_systems(Update, on_spawn_resource_pool);
    }
}

pub fn spawn_resource_pool(mut commands: Commands) {
    commands.spawn(ResourcePoolBundle::default());
}

#[derive(Component)]
pub struct UiResourcePool {}

pub fn on_spawn_resource_pool(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pools: Query<(&ResourcePool, &Player), Added<ResourcePool>>,
) {
    for (pool, player) in pools.iter() {
        commands
            .spawn((NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexEnd,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle {
                        style: Style {
                            justify_content: JustifyContent::FlexEnd,
                            bottom: Val::Px(10.0),
                            left: Val::Px(10.0),
                            position_type: PositionType::Absolute,
                            margin: UiRect {
                                left: Val::Px(10.0),
                                right: Val::Px(10.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(10.0),
                            },
                            ..default()
                        },
                        text: Text::from_section(
                            "mana".to_string() + &pool.mana.value.to_string(),
                            TextStyle {
                                font: asset_server.load("han_rounded.ttf"),
                                font_size: 50.0,
                                color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                            },
                        ),
                        ..default()
                    },
                    UiResourcePool {},
                ));
            });
    }
}

pub fn reg_resources() {}

#[derive(Bundle, Default)]
pub struct ResourcePoolBundle {
    pub resource_pool: ResourcePool,
    pub player: Player,
}

pub struct BasicResource {
    value: f32,
    reg: f32,
    max: f32,
    min: f32,
}

#[derive(Component)]
pub struct ResourcePool {
    mana: BasicResource,
}

impl BasicResource {
    fn new(value: f32, reg: f32, min: f32, max: f32) -> Self {
        Self {
            value,
            reg,
            min,
            max,
        }
    }
}

impl Default for ResourcePool {
    fn default() -> Self {
        Self {
            mana: BasicResource::new(10., 1., 0., 10.),
        }
    }
}
impl ResourcePool {}
