use std::str::Utf8Error;

use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
struct CardAsset(pub String);

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
            let data_str = std::str::from_utf8(bytes.as_slice())?;
            let asset = CardAsset(data_str.into());
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}

pub struct CardLoaderPlugin;

impl Plugin for CardLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CardAsset>()
            .init_asset_loader::<CardLoader>()
            .add_systems(Startup, load_cards);
    }
}

fn load_cards(asset_server: Res<AssetServer>) {
    let card_data: Handle<CardAsset> = asset_server.load("cards.toml");
    info!("{:?}", card_data);
}
