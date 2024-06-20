
use bevy::prelude::*;
use bevy_renet::{
    client_connected,
    renet::{
        transport::{ClientAuthentication, ServerAuthentication, ServerConfig},
        ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent,
    },
    transport::{NetcodeClientPlugin, NetcodeServerPlugin},
    RenetClientPlugin,
};

pub fn client_plugin(app: &mut App) {
    app
        .add_plugins(RenetClientPlugin);
}


pub enum ClientChannel {
    Input,
    Command,
}
