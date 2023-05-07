use bevy::prelude::*;
use texas_holdem_common::{Player, Round};

#[derive(Debug, Default, Resource)]
pub struct CurrentPlayInfo {
    pub play_id: Option<u64>,
    pub room_id: u64,
    pub round: Round,
    pub participants: Vec<Player>,
}
