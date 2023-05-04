use crate::RoomDTO;
use serde::{Deserialize, Serialize};

// 获取房间列表
pub const GET_ROOMS_CHANNEL_ID: u8 = 0;
// 创建房间
pub const CREATE_ROOM_CHANNEL_ID: u8 = 1;
// 进入房间
pub const ENTER_ROOT_CHANNEL_ID: u8 = 2;

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
    // resp
    pub room_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnterRoomMessage {
    pub timestamp: u64,
    // req
    pub room_id: u32,
    pub room_password: String,
    pub user_name: String,
    // resp
}