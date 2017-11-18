use super::model::*;
use super::*;

pub fn get_player_by_id(id: i32) -> Option<Player> {
    persistence::get_player_by_id(id)
}
