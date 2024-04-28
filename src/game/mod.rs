pub mod animate;
pub mod camera;
pub mod card;

use bevy::prelude::*;
use card::{CardBundle, CardPlugin};

use crate::{AppState, CardsHandle};

use self::{
    camera::PlayerCameraPlugin,
    card::{Card, CardInfo, CardStats, ISizeWithMax},
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
) {
    if asset_server.get_recursive_dependency_load_state(&cards.0) == 
    Some(bevy::asset::RecursiveDependencyLoadState::Loaded) {
        for (_, card_info) in card_infos.iter() {
            println!("{:?}", card_info)
        }
        state.set(AppState::Playing);
        // commands.spawn(CardBundle {
        //     transform: Transform::from_xyz(0.5, 0.0, 0.0),
        //     global_transform: default(),
        //     card: Card {
        //         animations: default(),
        //         info: CardInfo {
        //             card_type: card::CardType::Creature,
        //             stats: CardStats {
        //                 toughness: ISizeWithMax {
        //                     current: 0,
        //                     max: 10,
        //                 },
        //                 power: 0,
        //             },
        //             name: "hunman solider".into(),
        //             desc: "death will".into(),
        //         },
        //         z: 0,
        //     },
        // });
    }
}
