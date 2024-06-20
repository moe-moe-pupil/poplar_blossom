pub mod client;
pub mod server;
pub mod protocol;

use bevy::prelude::*;

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                client::client_plugin,
                server::server_plugin,
            ));
    }
}