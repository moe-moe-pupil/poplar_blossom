mod game;
mod toml;
use bevy::prelude::*;
use game::GamePlugin;
use toml::CardLoaderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CardLoaderPlugin)
        .add_plugins(GamePlugin)
        .run();
}
