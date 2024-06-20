
use bevy::prelude::*;
use bevy_renet::{
    client_connected,
    renet::{
        transport::{ClientAuthentication, ServerAuthentication, ServerConfig},
        ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent,
    },
    transport::{NetcodeClientPlugin, NetcodeServerPlugin},
    RenetServerPlugin,
};
use renet::{
    transport::{NetcodeClientTransport, NetcodeServerTransport, NetcodeTransportError},
    ClientId,
};

use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};

pub fn server_plugin(app: &mut App) {
    app
        .add_plugins(RenetServerPlugin);

    let server = RenetServer::new(ConnectionConfig::default());
    app.insert_resource(server);
}


pub enum ServerChannel {
    ServerMessages,
    NetworkedEntities,
}
