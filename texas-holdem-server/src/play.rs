use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use texas_holdem_common::{
    channel::{BroadcastPlayInfoMessage, BROADCAST_PLAY_INFO_CHANNEL_ID},
    util::timestamp,
    Player, PlayerRole, RoomState, Round,
};

use crate::room::RoomList;

// 一场对局
#[derive(Debug)]
pub struct Play {
    pub play_id: u64,
    pub room_id: u64,
    pub round: Round,
    pub participants: Vec<Player>,
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
            play_list.0.push(Play {
                play_id: timestamp(),
                room_id: room.room_id,
                round: Round::Start,
                participants: room
                    .players
                    .iter()
                    .filter(|player| player.player_role == PlayerRole::Participant)
                    .cloned()
                    .collect(),
            });
        }
    }
}

pub fn process_play_round_start(mut play_list: ResMut<PlayList>) {
    for play in play_list.0.iter_mut() {
        if play.round == Round::Start {
            play.round = Round::Preflop;
        }
    }
}
