use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{renet::{RenetClient, RenetConnectionConfig, ClientAuthentication}, RenetClientPlugin};
use texas_holdem_common::PROTOCOL_ID;

use crate::{network::{get_rooms}, table::{setup_table, setup_one_card}, room::RoomList};

mod network;
mod table;
mod room;

pub enum AppState {
    Lobby,
    Gaming,
}

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
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RenetClientPlugin::default())
        .insert_resource(new_renet_client())
        .insert_resource(RoomList(Vec::new()))
        .add_systems((get_rooms, ))
        .add_startup_systems((setup_table, setup_one_card, setup_camera))
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera2d_bundle = Camera2dBundle::default();
    camera2d_bundle.projection.scale = 2.5;
    commands.spawn(camera2d_bundle);
}