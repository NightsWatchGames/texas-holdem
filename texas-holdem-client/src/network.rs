use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use texas_holdem_common::{
    channel::{GetRoomsMessage, GET_ROOMS_CHANNEL_ID},
    util::timestamp,
};

use crate::room::RoomList;

pub fn get_rooms(
    mut client: ResMut<RenetClient>,
    mut room_list: ResMut<RoomList>,
    mut refresh_cd: Local<f32>,
    time: Res<Time>,
) {
    *refresh_cd -= time.delta_seconds();

    if *refresh_cd < 0.0 {
        let message = GetRoomsMessage {
            timestamp: timestamp(),
            rooms: Vec::new(),
        };
        client.send_message(GET_ROOMS_CHANNEL_ID, serde_json::to_vec(&message).unwrap());
        *refresh_cd = 5.0;
    }

    while let Some(message) = client.receive_message(GET_ROOMS_CHANNEL_ID) {
        if let Ok(message) = serde_json::from_slice::<GetRoomsMessage>(&message) {
            info!("Received get rooms message: {:?}", message);
            room_list.0 = message.rooms;
        }
    }
}
