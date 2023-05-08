use bevy::{prelude::*, utils::HashMap};
use bevy_renet::renet::RenetServer;
use texas_holdem_common::{
    channel::{BroadcastPlayInfoMessage, BROADCAST_PLAY_INFO_CHANNEL_ID},
    util::timestamp,
    Card, Player, PlayerRole, RoomState, Round,
};

use crate::room::RoomList;

// 一场对局
#[derive(Debug)]
pub struct Play {
    pub play_id: u64,
    pub room_id: u64,
    pub round: Round,
    pub participants: Vec<Player>,
    pub dealer_name: Option<String>,
    pub small_blind_name: Option<String>,
    pub big_blind_name: Option<String>,
    pub hole_cards: HashMap<String, [Card; 2]>,
    pub flop_cards: Option<[Card; 3]>,
    pub turn_card: Option<Card>,
    pub river_card: Option<Card>,
    pub card_pool: Vec<Card>,
    pub pot: u32,
}

#[derive(Debug, Default, Resource)]
pub struct PlayList(pub Vec<Play>);

pub fn broadcast_play_info(
    mut server: ResMut<RenetServer>,
    play_list: Res<PlayList>,
    room_list: Res<RoomList>,
    mut refresh_cd: Local<f32>,
    time: Res<Time>,
) {
    *refresh_cd -= time.delta_seconds();
    if *refresh_cd < 0.0 {
        for play in play_list.0.iter() {
            let message = BroadcastPlayInfoMessage {
                timestamp: timestamp(),
                room_id: play.room_id,
                play_id: play.play_id,
                round: play.round,
                participants: play.participants.clone(),
            };
            // 向房间内的所有玩家广播对局信息
            if let Some(room) = room_list.0.iter().find(|room| room.room_id == play.room_id) {
                for player in room.players.iter() {
                    server.send_message(
                        player.player_client_id,
                        BROADCAST_PLAY_INFO_CHANNEL_ID,
                        serde_json::to_vec(&message).unwrap(),
                    );
                }
            }
        }
        // 1秒广播一次
        *refresh_cd = 1.0;
    }
}

pub fn start_new_play(room_list: Res<RoomList>, mut play_list: ResMut<PlayList>) {
    for room in room_list.0.iter() {
        // 如果房间正在游戏中，且对局列表中没有该房间的对局，则创建新的对局
        if room.room_state == RoomState::Playing
            && play_list
                .0
                .iter()
                .find(|play| play.room_id == room.room_id)
                .is_none()
        {
            let play = Play {
                play_id: timestamp(),
                room_id: room.room_id,
                round: Round::Start,
                participants: room
                    .players
                    .iter()
                    .filter(|player| player.player_role == PlayerRole::Participant)
                    .cloned()
                    .collect(),
                dealer_name: None,
                small_blind_name: None,
                big_blind_name: None,
                hole_cards: HashMap::new(),
                flop_cards: None,
                turn_card: None,
                river_card: None,
                card_pool: Card::pool(),
                pot: 0,
            };
            // 参与者人数大于等于3人才开始游戏
            // TODO 筹码检查
            if play.participants.len() >= 3 {
                play_list.0.push(play);
            }
        }
    }
}

pub fn process_play_round_start(mut play_list: ResMut<PlayList>, room_list: Res<RoomList>) {
    for play in play_list.0.iter_mut() {
        if play.round != Round::Start {
            continue;
        }
        if let Some(room) = room_list.0.iter().find(|room| room.room_id == play.room_id) {
            if room.room_state != RoomState::Playing {
                continue;
            }
            // 确定庄家以及大盲注和小盲注位置
            if let Some(last_dealer_name) = room.last_dealer_name.as_ref() {
                if let Some(last_dealer_index) = play
                    .participants
                    .iter()
                    .position(|player| player.player_name == *last_dealer_name)
                {
                    play.dealer_name = Some(
                        play.participants[(last_dealer_index + 1) % play.participants.len()]
                            .player_name
                            .clone(),
                    );
                    play.small_blind_name = Some(
                        play.participants[(last_dealer_index + 2) % play.participants.len()]
                            .player_name
                            .clone(),
                    );
                    play.big_blind_name = Some(
                        play.participants[(last_dealer_index + 3) % play.participants.len()]
                            .player_name
                            .clone(),
                    );
                } else {
                    play.dealer_name = Some(play.participants[0].player_name.clone());
                    play.small_blind_name = Some(play.participants[1].player_name.clone());
                    play.big_blind_name = Some(play.participants[2].player_name.clone());
                }
            } else {
                play.dealer_name = Some(play.participants[0].player_name.clone());
                play.small_blind_name = Some(play.participants[1].player_name.clone());
                play.big_blind_name = Some(play.participants[2].player_name.clone());
            }
            play.round = Round::Preflop;
        }
    }
}

pub fn process_play_round_preflop(mut play_list: ResMut<PlayList>, room_list: Res<RoomList>) {
    for play in play_list.0.iter_mut() {
        if play.round != Round::Preflop {
            continue;
        }
        if let Some(room) = room_list.0.iter().find(|room| room.room_id == play.room_id) {
            if room.room_state != RoomState::Playing {
                continue;
            }
            // 大小盲注（大盲注为最小下注金额、小盲注为最小下注金额一半）
            // 发手牌
            // 下注

            play.round = Round::Flop;
        }
    }
}
