pub mod animate;
pub mod camera;
pub mod card;

use bevy::prelude::*;
use card::{CardBundle, CardPlugin};

use self::{
    camera::PlayerCameraPlugin,
    card::{Card, CardInfo, CardStats, ISizeWithMax},
};
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CardPlugin)
            .add_plugins(PlayerCameraPlugin)
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(CardBundle {
        transform: Transform::from_xyz(0.5, 0.0, 0.0),
        global_transform: default(),
        card: Card {
            animations: default(),
            info: CardInfo {
                card_type: card::CardType::Creature,
                stats: CardStats {
                    toughness: ISizeWithMax {
                        current: 0,
                        max: 10,
                    },
                    power: ISizeWithMax { current: 0, max: 0 },
                },
            },
            z: 0,
        },
    });
}
