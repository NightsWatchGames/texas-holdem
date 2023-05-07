use crate::{Player, PlayerRole, RoomDTO};
use serde::{Deserialize, Serialize};

// 获取房间列表
pub const GET_ROOMS_CHANNEL_ID: u8 = 0;
// 创建房间
pub const CREATE_ROOM_CHANNEL_ID: u8 = 1;
// 进入房间
pub const ENTER_ROOT_CHANNEL_ID: u8 = 2;
// 切换角色
pub const SWITCH_PLAYER_ROLE_CHANNEL_ID: u8 = 3;
// 房间信息
pub const BROADCAST_ROOM_INFO_CHANNEL_ID: u8 = 4;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRoomsMessage {
    pub timestamp: u64,
    // req
    pub rooms: Vec<RoomDTO>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomMessage {
    pub timestamp: u64,
    // req
    pub room_name: String,
    pub room_password: String,
    pub player_name: String,
    // resp
    pub room_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnterRoomMessage {
    pub timestamp: u64,
    // req
    pub room_id: u64,
    pub room_password: String,
    pub player_name: String,
    // resp
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwitchPlayerRoleMessage {
    pub timestamp: u64,
    // req
    pub room_id: u64,
    pub target_player_role: PlayerRole,
    // resp
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastRoomInfoMessage {
    pub timestamp: u64,
    pub room_id: u64,
    pub room_name: String,
    pub players: Vec<Player>,
    // pub operation_log: Vec<String>,
}
