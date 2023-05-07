use std::{default, time::Duration};

use bevy_renet::renet::{
    ChannelConfig, ReliableChannelConfig, RenetConnectionConfig, UnreliableChannelConfig,
};
use channel::{
    BROADCAST_ROOM_INFO_CHANNEL_ID, CREATE_ROOM_CHANNEL_ID, ENTER_ROOT_CHANNEL_ID,
    GET_ROOMS_CHANNEL_ID, SWITCH_PLAYER_ROLE_CHANNEL_ID,
};
use serde::{Deserialize, Serialize};

pub mod channel;
pub mod util;

pub const PROTOCOL_ID: u64 = 0;

pub fn connection_config() -> RenetConnectionConfig {
    let channels_config = vec![
        ChannelConfig::Reliable(ReliableChannelConfig {
            channel_id: GET_ROOMS_CHANNEL_ID,
            ..Default::default()
        }),
        ChannelConfig::Reliable(ReliableChannelConfig {
            channel_id: CREATE_ROOM_CHANNEL_ID,
            ..Default::default()
        }),
        ChannelConfig::Reliable(ReliableChannelConfig {
            channel_id: ENTER_ROOT_CHANNEL_ID,
            ..Default::default()
        }),
        ChannelConfig::Reliable(ReliableChannelConfig {
            channel_id: SWITCH_PLAYER_ROLE_CHANNEL_ID,
            ..Default::default()
        }),
        ChannelConfig::Unreliable(UnreliableChannelConfig {
            channel_id: BROADCAST_ROOM_INFO_CHANNEL_ID,
            ..Default::default()
        }),
    ];

    RenetConnectionConfig {
        max_packet_size: 16 * 1024,
        sent_packets_buffer_size: 256,
        received_packets_buffer_size: 256,
        reassembly_buffer_size: 256,
        rtt_smoothing_factor: 0.005,
        packet_loss_smoothing_factor: 0.1,
        bandwidth_smoothing_factor: 0.1,
        heartbeat_time: Duration::from_millis(100),
        send_channels_config: channels_config.clone(),
        receive_channels_config: channels_config,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomDTO {
    // 房间id
    pub room_id: u64,
    // 房间名称
    pub room_name: String,
    // 房主名称
    pub owner_name: String,
    // 房间人数
    pub player_count: u32,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum RoomState {
    // 等待中
    #[default]
    Waiting,
    // 游戏中
    Playing,
    // 暂停中
    Paused,
}

impl RoomState {
    pub fn name(&self) -> &'static str {
        match self {
            RoomState::Waiting => "Waiting",
            RoomState::Playing => "Playing",
            RoomState::Paused => "Paused",
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum Round {
    #[default]
    Start,
    Preflop,
    Flop,
    Turn,
    River,
    End,
}
impl Round {
    pub fn name(&self) -> &'static str {
        match self {
            Round::Start => "Start",
            Round::Preflop => "Preflop",
            Round::Flop => "Flop",
            Round::Turn => "Turn",
            Round::River => "River",
            Round::End => "End",
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerRole {
    // 旁观者
    #[default]
    Spectator,
    // 参与者
    Participant,
}

impl PlayerRole {
    pub fn name(&self) -> &'static str {
        match self {
            PlayerRole::Spectator => "Spectator",
            PlayerRole::Participant => "Participant",
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Player {
    pub player_client_id: u64,
    pub player_name: String,
    pub player_role: PlayerRole,
}
