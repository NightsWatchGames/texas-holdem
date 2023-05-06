use bevy::prelude::*;
use texas_holdem_common::PlayerRole;

#[derive(Debug)]
pub struct Player {
    pub player_client_id: u64,
    pub player_name: String,
    pub player_role: PlayerRole,
}

#[derive(Debug)]
pub struct Room {
    pub room_id: u64,
    pub room_name: String,
    pub room_password: String,
    pub room_state: RoomState,
    pub owner_client_id: u64,
    pub players: Vec<Player>,
}

// 房间列表
#[derive(Debug, Resource)]
pub struct RoomList(pub Vec<Room>);

impl Room {
    pub fn owner_name(&self) -> String {
        self.players
            .iter()
            .find(|user| user.player_client_id == self.owner_client_id)
            .unwrap()
            .player_name
            .clone()
    }
}

#[derive(Debug)]
pub enum RoomState {
    // 等待中
    Waiting,
    // 游戏中
    Playing,
    // 暂停中
    Paused,
}

// 一场对局
#[derive(Debug)]
pub struct Play {
    pub play_id: u64,
    pub room_id: u64,
    pub participants: Vec<Player>,
}
