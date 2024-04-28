mod game;
use bevy::prelude::*;
use bevy_common_assets::csv::{CsvAssetPlugin, LoadedCsv};
use game::{card::CardInfo, GamePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CsvAssetPlugin::<CardInfo>::new(&["cards.csv"]))
        // .add_plugins(CardLoaderPlugin)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins(GamePlugin)
        .run();
}

#[derive(Resource)]
pub struct CardsHandle(Handle<LoadedCsv<CardInfo>>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cards = CardsHandle(asset_server.load("cards.csv"));
    commands.insert_resource(cards);
}


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    LoadingCards,
    Playing,
}
