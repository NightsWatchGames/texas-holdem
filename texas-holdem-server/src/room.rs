use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use texas_holdem_common::{
    channel::{
        BroadcastRoomInfoMessage, SetRoomStateMessage, SwitchPlayerRoleMessage,
        BROADCAST_ROOM_INFO_CHANNEL_ID, SET_ROOM_STATE_CHANNEL_ID, SWITCH_PLAYER_ROLE_CHANNEL_ID,
    },
    util::timestamp,
    Player, PlayerRole, RoomState,
};

#[derive(Debug)]
pub struct Room {
    pub room_id: u64,
    pub room_name: String,
    pub room_password: String,
    pub room_state: RoomState,
    pub owner_name: String,
    pub players: Vec<Player>,
}
impl Room {
    pub fn contains_player(&self, player_name: &str) -> bool {
        self.players
            .iter()
            .any(|player| player.player_name == player_name)
    }
}

// 房间列表
#[derive(Debug, Resource)]
pub struct RoomList(pub Vec<Room>);

pub fn handle_switch_player_role(mut server: ResMut<RenetServer>, mut room_list: ResMut<RoomList>) {
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, SWITCH_PLAYER_ROLE_CHANNEL_ID) {
            if let Ok(mut message) = serde_json::from_slice::<SwitchPlayerRoleMessage>(&message) {
                info!("Received switch player role message: {:?}", message);
                if let Some(room) = room_list
                    .0
                    .iter_mut()
                    .find(|room| room.room_id == message.room_id)
                {
                    room.players
                        .iter_mut()
                        .find(|player| player.player_client_id == client_id)
                        .unwrap()
                        .player_role = message.target_player_role;
                    message.success = true;
                    server.send_message(
                        client_id,
                        SWITCH_PLAYER_ROLE_CHANNEL_ID,
                        serde_json::to_vec(&message).unwrap(),
                    );
                } else {
                    error!("Room not found when switch player role")
                }
            }
        }
    }
}

pub fn broadcast_room_info(
    mut server: ResMut<RenetServer>,
    room_list: Res<RoomList>,
    mut refresh_cd: Local<f32>,
    time: Res<Time>,
) {
    *refresh_cd -= time.delta_seconds();
    if *refresh_cd < 0.0 {
        for room in room_list.0.iter() {
            let message = BroadcastRoomInfoMessage {
                timestamp: timestamp(),
                room_id: room.room_id,
                room_name: room.room_name.clone(),
                room_state: room.room_state,
                players: room.players.clone(),
            };
            for player in room.players.iter() {
                server.send_message(
                    player.player_client_id,
                    BROADCAST_ROOM_INFO_CHANNEL_ID,
                    serde_json::to_vec(&message).unwrap(),
                );
            }
        }
        // 5秒广播一次
        *refresh_cd = 5.0;
    }
}

pub fn handle_set_room_state(mut server: ResMut<RenetServer>, mut room_list: ResMut<RoomList>) {
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, SET_ROOM_STATE_CHANNEL_ID) {
            if let Ok(mut message) = serde_json::from_slice::<SetRoomStateMessage>(&message) {
                info!("Received set room state message: {:?}", message);
                if let Some(room) = room_list
                    .0
                    .iter_mut()
                    .find(|room| room.room_id == message.room_id)
                {
                    if room.owner_name == message.player_name {
                        room.room_state = message.target_room_state;
                        message.success = true;
                    } else {
                        message.success = false;
                    }
                    server.send_message(
                        client_id,
                        SET_ROOM_STATE_CHANNEL_ID,
                        serde_json::to_vec(&message).unwrap(),
                    );
                } else {
                    error!("Room not found when set room state")
                }
            }
        }
    }
}
