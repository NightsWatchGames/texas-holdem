use std::{net::UdpSocket, time::SystemTime};

use bevy::{log::LogPlugin, prelude::*};
use bevy_renet::{
    renet::{RenetServer, ServerAuthentication, ServerConfig},
    RenetServerPlugin,
};
use lobby::{handle_create_room, handle_enter_room, handle_get_rooms};
use play::{broadcast_play_info, process_play_round_start, start_new_play, PlayList};
use room::{broadcast_room_info, handle_set_room_state, handle_switch_player_role};
use texas_holdem_common::{connection_config, PROTOCOL_ID};

use crate::{network::handle_events_system, room::RoomList};

mod lobby;
mod network;
mod play;
mod room;

fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config =
        ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    RenetServer::new(current_time, server_config, connection_config(), socket).unwrap()
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(new_renet_server())
        .insert_resource(RoomList(Vec::new()))
        .insert_resource(PlayList(Vec::new()))
        .add_systems((
            handle_get_rooms,
            handle_create_room,
            handle_enter_room,
            handle_switch_player_role,
            broadcast_room_info,
            handle_set_room_state,
            handle_events_system,
            broadcast_play_info,
            start_new_play,
            process_play_round_start,
        ))
        .run();
}
