mod game;
use bevy::prelude::*;
use bevy_common_assets::csv::{CsvAssetPlugin, LoadedCsv};
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use game::{card::CardInfo, GamePlugin};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.4,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CsvAssetPlugin::<CardInfo>::new(&["cards.csv"]))
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
