use bevy::prelude::*;
use texas_holdem_common::RoomDTO;

// 房间列表
#[derive(Debug, Resource)]
pub struct RoomList(pub Vec<RoomDTO>);