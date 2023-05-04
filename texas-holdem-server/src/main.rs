use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{RenetServerPlugin, renet::{RenetServer, RenetConnectionConfig, ServerConfig, ServerAuthentication, ServerEvent}};
use room::{Room, Player};
use texas_holdem_common::PROTOCOL_ID;

use crate::{network::{handle_events_system, handle_get_rooms}, room::RoomList};

mod network;
mod room;

fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn main() {
    let mock = Room {
        room_id: 0,
        room_name: "mock".to_string(),
        room_password: "".to_string(),
        owner_client_id: 0,
        players: vec![Player {
            player_client_id: 0,
            player_name: "mock".to_string(),
            player_role: room::PlayerRole::Participant,
        }],
    };
    let room_list = RoomList(vec![mock]);
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(new_renet_server())
        // .insert_resource(RoomList(Vec::new()))
        .insert_resource(room_list)
        .add_systems((handle_get_rooms, handle_events_system))
        .run();
}