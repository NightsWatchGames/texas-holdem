use std::{default, time::Duration};

use bevy_renet::renet::{
    ChannelConfig, ReliableChannelConfig, RenetConnectionConfig, UnreliableChannelConfig,
};
use channel::{
    BROADCAST_PLAY_INFO_CHANNEL_ID, BROADCAST_ROOM_INFO_CHANNEL_ID, CREATE_ROOM_CHANNEL_ID,
    ENTER_ROOT_CHANNEL_ID, GET_ROOMS_CHANNEL_ID, SET_ROOM_STATE_CHANNEL_ID,
    SWITCH_PLAYER_ROLE_CHANNEL_ID,
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
        ChannelConfig::Reliable(ReliableChannelConfig {
            channel_id: SET_ROOM_STATE_CHANNEL_ID,
            ..Default::default()
        }),
        ChannelConfig::Unreliable(UnreliableChannelConfig {
            channel_id: BROADCAST_PLAY_INFO_CHANNEL_ID,
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
    // 房间状态
    pub room_state: RoomState,
    // 房主名称
    pub owner_name: String,
    // 房间人数
    pub player_count: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Round {
    #[default]
    // 开始（座位分配，确定庄家和大小盲注）
    Start,
    // 翻牌前（大小盲注，每人发出2张底牌，下注）
    Preflop,
    // 翻牌圈（发出三张公共牌，下注）
    Flop,
    // 转牌圈（发出第四张公共牌，下注）
    Turn,
    // 河牌圈（发出第五张公共牌，下注）
    River,
    // 摊牌（比较大小、分钱）
    Showdown,
    // 结束（记录本局庄家位置）
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
            Round::Showdown => "Showdown",
            Round::End => "End",
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    pub chips: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Card {
    // 黑桃
    Spade(u8),
    // 红桃
    Heart(u8),
    // 梅花
    Club(u8),
    // 方块
    Diamond(u8),
}
impl Card {
    pub fn pool() -> Vec<Card> {
        let mut pool = Vec::with_capacity(52);
        for i in 1..=13 {
            pool.push(Card::Spade(i));
            pool.push(Card::Heart(i));
            pool.push(Card::Club(i));
            pool.push(Card::Diamond(i));
        }
        pool
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoundAction {
    // 过牌
    Check,
    // 下注
    Bet,
    // 跟注
    Call,
    // 加注
    Raise,
    // 弃牌
    Fold,
    // 全下
    AllIn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    // 庄家
    Dealer,
    // 小盲注
    SmallBlind,
    // 大盲注
    BigBlind,
    // 其他
    Other,
}
