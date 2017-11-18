use super::model::*;
use super::*;

pub fn get_player_by_id(id: usize) -> Option<Player> {
    persistence::get_player_by_id(id)
}
