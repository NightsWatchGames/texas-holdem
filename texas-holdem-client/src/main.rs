use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{renet::{RenetClient, RenetConnectionConfig, ClientAuthentication}, RenetClientPlugin};
use texas_holdem_common::PROTOCOL_ID;

fn new_renet_client() -> RenetClient {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
}

fn main() {
    println!("Hello world client!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RenetClientPlugin::default())
        .insert_resource(new_renet_client())
        .add_systems((send_testing_message, receive_message_system))
        .run();
}

fn send_testing_message(mut client: ResMut<RenetClient>) {
    let channel_id = 0;
     // Send a text message to the server
    client.send_message(channel_id, "client sending testing message".as_bytes().to_vec());
}

fn receive_message_system(mut client: ResMut<RenetClient>) {
    let channel_id = 0;
    while let Some(message) = client.receive_message(channel_id) {
        // Handle received message
        println!("Received message: {:?}", String::from_utf8(message));
    }
}