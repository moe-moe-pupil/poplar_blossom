use std::str::Utf8Error;

use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use serde::Deserialize;
use thiserror::Error;

use crate::game::card::CardInfo;

#[derive(Asset, TypePath, Debug, Deserialize)]
struct CardAsset(pub CardInfo);

#[derive(Default)]
struct CardLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
enum CardAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Utf8Error")]
    Utf8Error(#[from] Utf8Error),
}

impl AssetLoader for CardLoader {
    type Asset = CardAsset;
    type Settings = ();
    type Error = CardAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let data_str = csv::Reader::from_reader(bytes.as_slice());
            for result in rdr.deserialize() {
                // Notice that we need to provide a type hint for automatic
                // deserialization.
                let record: Record = result?;
                println!("{:?}", record);
            }
            let asset = CardAsset(data_str.into());
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["csv"]
    }
}

pub struct CardLoaderPlugin;

impl Plugin for CardLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CardAsset>()
            .init_asset_loader::<CardLoader>()
            .add_systems(Startup, load_cards)
            .add_systems(Update, load_data);
    }
}

fn load_cards(mut commands: Commands,asset_server: Res<AssetServer>) {
    let card_data: Handle<CardAsset> = asset_server.load("cards.csv");
    info!("{:?}", card_data);
    commands.insert_resource(DataAssets { handle: card_data} );
}

#[derive(Debug, Resource)]
struct DataAssets {
    pub handle: Handle<CardAsset>,
}

fn load_data(
    keys: Res<ButtonInput<KeyCode>>,
    toml_assets: Res<Assets<CardAsset>>,
    data_assets: Res<DataAssets>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let data = toml_assets
            .get(&data_assets.handle)
            .expect("Not a valid asset!");
        info!("{:?}", data);
    }
}
