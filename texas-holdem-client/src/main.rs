use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_renet::{
    renet::{ClientAuthentication, RenetClient},
    RenetClientPlugin,
};
use lobby::{
    lobby_create_room_ui, lobby_enter_room_modal_ui, lobby_room_list_ui, lobby_set_player_name_ui,
    CreateRoomEvent, EnterRoomEvent, InputPasswordModalOpen, NewRoomSettings, PlayerName, RoomList,
    RoomToEnter,
};
use network::{create_room, enter_room, receive_room_info, set_room_state, switch_player_role};
use play::CurrentPlayInfo;
use room::{
    play_round_ui_system, player_list_ui_system, player_role_ui_system, room_state_ui_system,
    set_room_state_ui_system, setup_room_ui, CurrentRoomInfo, SetRoomStateEvent,
    SwitchPlayerRoleEvent,
};
use texas_holdem_common::{connection_config, util::timestamp, PROTOCOL_ID};

use crate::{
    network::get_rooms,
    table::{setup_one_card, setup_table},
};

mod lobby;
mod network;
mod play;
mod room;
mod table;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum AppState {
    #[default]
    Lobby,
    Gaming,
}

fn new_renet_client() -> RenetClient {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    RenetClient::new(current_time, socket, connection_config(), authentication).unwrap()
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RenetClientPlugin::default())
        .add_plugin(EguiPlugin)
        .add_state::<AppState>()
        .add_event::<CreateRoomEvent>()
        .add_event::<EnterRoomEvent>()
        .add_event::<SwitchPlayerRoleEvent>()
        .add_event::<SetRoomStateEvent>()
        .insert_resource(new_renet_client())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RoomList(Vec::new()))
        .insert_resource(PlayerName(format!("Player{}", timestamp())))
        .insert_resource(NewRoomSettings::default())
        .insert_resource(RoomToEnter::default())
        .insert_resource(InputPasswordModalOpen::default())
        .insert_resource(CurrentRoomInfo::default())
        .insert_resource(CurrentPlayInfo::default())
        .add_startup_systems((setup_camera,))
        .add_systems(
            (
                get_rooms,
                create_room,
                enter_room,
                lobby_room_list_ui,
                lobby_enter_room_modal_ui,
                lobby_create_room_ui,
                lobby_set_player_name_ui,
            )
                .in_set(OnUpdate(AppState::Lobby)),
        )
        .add_systems(
            (setup_table, setup_one_card, setup_room_ui).in_schedule(OnEnter(AppState::Gaming)),
        )
        .add_systems(
            (
                player_role_ui_system,
                player_list_ui_system,
                room_state_ui_system,
                play_round_ui_system,
                set_room_state_ui_system,
                switch_player_role,
                receive_room_info,
                set_room_state,
            )
                .in_set(OnUpdate(AppState::Gaming)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera2d_bundle = Camera2dBundle::default();
    camera2d_bundle.projection.scale = 2.5;
    commands.spawn(camera2d_bundle);
}
