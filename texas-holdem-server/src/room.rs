use bevy::prelude::*;
use texas_holdem_common::{Player, PlayerRole, RoomState};

#[derive(Debug)]
pub struct Room {
    pub room_id: u64,
    pub room_name: String,
    pub room_password: String,
    pub room_state: RoomState,
    pub owner_name: String,
    pub players: Vec<Player>,
}

// 房间列表
#[derive(Debug, Resource)]
pub struct RoomList(pub Vec<Room>);

// 一场对局
#[derive(Debug)]
pub struct Play {
    pub play_id: u64,
    pub room_id: u64,
    pub participants: Vec<Player>,
}
