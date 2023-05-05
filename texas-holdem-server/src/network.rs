use bevy::prelude::*;
use bevy_renet::renet::{RenetServer, ServerEvent};
use texas_holdem_common::{
    channel::{GetRoomsMessage, GET_ROOMS_CHANNEL_ID},
    RoomDTO,
};

use crate::room::RoomList;

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
