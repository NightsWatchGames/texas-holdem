use bevy::prelude::*;
use bevy_renet::renet::{RenetServer, ServerEvent};
use texas_holdem_common::{
    channel::{
        BroadcastRoomInfoMessage, CreateRoomMessage, EnterRoomMessage, GetRoomsMessage,
        SetRoomStateMessage, SwitchPlayerRoleMessage, BROADCAST_ROOM_INFO_CHANNEL_ID,
        CREATE_ROOM_CHANNEL_ID, ENTER_ROOT_CHANNEL_ID, GET_ROOMS_CHANNEL_ID,
        SET_ROOM_STATE_CHANNEL_ID, SWITCH_PLAYER_ROLE_CHANNEL_ID,
    },
    util::timestamp,
    Player, PlayerRole, RoomDTO, RoomState,
};

use crate::room::{Room, RoomList};

pub fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, user_data) => {
                println!("Client {} connected", id);
            }
            ServerEvent::ClientDisconnected(id) => {
                // TODO 房主断开需要进行异常处理
                println!("Client {} disconnected", id);
            }
        }
    }
}
