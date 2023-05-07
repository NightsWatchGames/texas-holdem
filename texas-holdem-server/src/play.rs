use bevy::prelude::*;
use texas_holdem_common::{Player, Round};

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
