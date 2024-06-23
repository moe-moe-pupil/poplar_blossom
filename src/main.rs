mod game;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::csv::{CsvAssetPlugin, LoadedCsv};
use bevy_editor_pls::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

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
    .add_loading_state(
        LoadingState::new(AppState::Loading)
            .continue_to_state(AppState::MainMenu)
            .load_collection::<Models>(),
    )
    .add_systems(Startup, setup)
    .add_plugins(GamePlugin);

    #[cfg(debug_assertions)]
    {
        app.add_plugins((
            // LogDiagnosticsPlugin::default(),
            RapierDebugRenderPlugin::default(),
            // WorldInspectorPlugin::new(),
            EditorPlugin::default(),
        ));
    }

    app.run();
}

#[derive(AssetCollection, Resource)]
pub struct Models {
    #[asset(path = "battlefield.glb")]
    pub battlefield_model: Handle<Gltf>,
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
