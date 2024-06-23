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

    app
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
    .add_systems(Update, (spawn_level.run_if(in_state(AppState::Loading)),))
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

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
/// helper marker component
pub struct LoadedMarker;

fn spawn_level(
    mut commands: Commands,
    scene_markers: Query<&LoadedMarker>,
    mut asset_event_reader: EventReader<AssetEvent<Gltf>>,
    mut next_state: ResMut<NextState<AppState>>,
    models: Res<Assets<bevy::gltf::Gltf>>,
) {
    if let Some(asset_event) = asset_event_reader.read().next() {
        if let AssetEvent::Added { id } = asset_event {
            info!("GLTF loaded/ added {:?}", asset_event);
            let my_gltf = models.get(*id).unwrap();
            if scene_markers.is_empty() {
                info!("spawning scene");
                commands.spawn((
                    SceneBundle {
                        scene: my_gltf.scenes[0].clone(),
                        ..default()
                    },
                    LoadedMarker,
                    Name::new("Battlefield"),
                ));
                next_state.set(AppState::MainMenu);
            }
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct Models {
    #[asset(path = "Battlefield.glb")]
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
