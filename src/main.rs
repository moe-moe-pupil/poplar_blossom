mod game;
use bevy::prelude::*;
use bevy_common_assets::csv::{CsvAssetPlugin, LoadedCsv};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::{card::CardInfo, GamePlugin};

fn main() {
    let mut app = App::new();

    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.4,
    })
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "CardPong".to_string(),
            resizable: true,
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resolution: WindowResolution::new(1080., 1360.),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(CsvAssetPlugin::<CardInfo>::new(&["cards.csv"]))
    .init_state::<AppState>()
    .add_systems(Startup, setup)
    .add_plugins(GamePlugin);
    
    #[cfg(debug_assertions)]
    {
        app.add_plugins((
            // LogDiagnosticsPlugin::default(),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
        ));
    }
    
    app.run();

}

#[derive(Resource)]
pub struct CardsHandle(Handle<LoadedCsv<CardInfo>>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cards = CardsHandle(asset_server.load("cards.csv"));
    let id = cards.0.id();
    commands.insert_resource(cards);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    RoomMenu,
    GameOverMenu,
}
