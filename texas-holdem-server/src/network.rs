use bevy::prelude::*;
use bevy_renet::renet::{RenetServer, ServerEvent};
use texas_holdem_common::{
    channel::{
        CreateRoomMessage, EnterRoomMessage, GetRoomsMessage, CREATE_ROOM_CHANNEL_ID,
        ENTER_ROOT_CHANNEL_ID, GET_ROOMS_CHANNEL_ID,
    },
    util::timestamp,
    RoomDTO,
};

use crate::room::{Player, PlayerRole, Room, RoomList};

pub fn handle_get_rooms(mut server: ResMut<RenetServer>, room_list: Res<RoomList>) {
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, GET_ROOMS_CHANNEL_ID) {
            if let Ok(mut message) = serde_json::from_slice::<GetRoomsMessage>(&message) {
                info!("Received get rooms message: {:?}", message);
                message.rooms = room_list
                    .0
                    .iter()
                    .map(|room| RoomDTO {
                        room_id: room.room_id,
                        room_name: room.room_name.clone(),
                        owner_name: room.owner_name(),
                        player_count: room.players.len() as u32,
                    })
                    .collect();
                server.send_message(
                    client_id,
                    GET_ROOMS_CHANNEL_ID,
                    serde_json::to_vec(&message).unwrap(),
                );
            }
        }
    }
}

pub fn handle_create_room(mut server: ResMut<RenetServer>, mut room_list: ResMut<RoomList>) {
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, CREATE_ROOM_CHANNEL_ID) {
            if let Ok(mut message) = serde_json::from_slice::<CreateRoomMessage>(&message) {
                info!("Received create room message: {:?}", message);
                message.room_id = timestamp();
                room_list.0.push(Room {
                    room_id: message.room_id,
                    room_name: message.room_name.clone(),
                    room_password: message.room_password.clone(),
                    owner_client_id: client_id,
                    players: vec![Player {
                        player_client_id: client_id,
                        player_name: message.player_name.clone(),
                        player_role: PlayerRole::Participant,
                    }],
                });
                server.send_message(
                    client_id,
                    CREATE_ROOM_CHANNEL_ID,
                    serde_json::to_vec(&message).unwrap(),
                );
            }
        }
    }
}

pub fn handle_enter_room(mut server: ResMut<RenetServer>, mut room_list: ResMut<RoomList>) {
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, ENTER_ROOT_CHANNEL_ID) {
            if let Ok(mut message) = serde_json::from_slice::<EnterRoomMessage>(&message) {
                info!("Received enter room message: {:?}", message);
                if let Some(room) = room_list
                    .0
                    .iter_mut()
                    .find(|room| room.room_id == message.room_id)
                {
                    room.players.push(Player {
                        player_client_id: client_id,
                        player_name: message.player_name.clone(),
                        player_role: PlayerRole::Participant,
                    });
                    message.success = true;
                    server.send_message(
                        client_id,
                        ENTER_ROOT_CHANNEL_ID,
                        serde_json::to_vec(&message).unwrap(),
                    );
                }
            }
        }
    }
}

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
