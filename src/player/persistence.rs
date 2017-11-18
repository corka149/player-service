use std::collections::HashMap;
use super::model::Player;
use super::Gender;

pub fn get_player_by_id(id: usize) -> Option<Player> {
    let src = set_up();
    match src.get(&id) {
        Some(player) => Some(player.clone()),
        None => None,
    }
}

fn set_up() -> HashMap<usize, Player> {
    let player_a = Player::new(1, String::from("Bastian"), Gender::Male, 27);
    let player_b = Player::new(2, String::from("Maria"), Gender::Female, 26);
    let player_c = Player::new(3, String::from("Corina"), Gender::Female, 25);
    let mut source = HashMap::new();

    source.insert(1, player_a);
    source.insert(2, player_b);
    source.insert(3, player_c);

    source
}
