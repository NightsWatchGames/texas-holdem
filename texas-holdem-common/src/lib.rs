use serde::{Deserialize, Serialize};

pub mod channel;
pub mod util;

pub const PROTOCOL_ID: u64 = 0;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomDTO {
    // 房间id
    pub room_id: u32,
    // 房间名称
    pub room_name: String,
    // 房主名称
    pub owner_name: String,
    // 房间人数
    pub player_count: u32,
}
