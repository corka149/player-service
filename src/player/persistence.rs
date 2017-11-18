use super::model::Player;
use diesel::prelude::*;

// Database
use super::super::database::*;

pub fn get_player_by_id(player_id: i32) -> Option<Player> {
    use super::super::schema::players::dsl::*;

    let mut result: Option<Player> = None;
    let connection = establish_connection();

    let results = players
        .filter(id.eq(player_id))
        .limit(1)
        .load::<Player>(&connection)
        .expect("Error loading player");

    for player in results {
        result = Some(player);
    }
    result
}
