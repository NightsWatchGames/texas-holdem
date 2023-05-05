use bevy::prelude::*;

#[derive(Debug)]
pub struct Player {
    pub player_client_id: u64,
    pub player_name: String,
    pub player_role: PlayerRole,
}

#[derive(Debug)]
pub enum PlayerRole {
    Spectator,
    Participant,
}

#[derive(Debug)]
pub struct Room {
    pub room_id: u64,
    pub room_name: String,
    pub room_password: String,
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
