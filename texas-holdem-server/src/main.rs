use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{RenetServerPlugin, renet::{RenetServer, RenetConnectionConfig, ServerConfig, ServerAuthentication, ServerEvent}};
use texas_holdem_common::PROTOCOL_ID;

fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn main() {
    println!("Hello world server!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(new_renet_server())
        .add_systems((send_testing_message, receive_message_system, handle_events_system))
        .run();
}

fn send_testing_message(mut server: ResMut<RenetServer>) {
    let channel_id = 0;
     // Send a text message for all clients
    server.broadcast_message(channel_id, "server sending testing message".as_bytes().to_vec());
}

fn receive_message_system(mut server: ResMut<RenetServer>) {
    let channel_id = 0;
     // Send a text message for all clients
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, channel_id) {
            // Handle received message
            println!("Received message: {:?}", String::from_utf8(message));
        }
    }
}

fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, user_data) => {
                println!("Client {} connected", id);
            }
            ServerEvent::ClientDisconnected(id) => {
                println!("Client {} disconnected", id);
            }
        }
    }
}