use iron::prelude::*;
use iron::method::*;
use iron::status;
use router::Router;
use serde_json;
use super::*;
use super::model::Player;


pub fn handler(req: &mut Request) -> IronResult<Response> {
    let possible_id = req.extensions.get::<Router>().unwrap().find("id");

    let p_json = match extract_id(possible_id) {
        Some(id) => extract_player_str(id),
        None => String::from(""),
    };

    match req.method {
        Get => Ok(Response::with((status::Ok, p_json))),
        _ => panic!("Not supported method"),
    }
}

//----------------------------------------------------------------------------------
//-------------------------------P R I V A T E--------------------------------------
//----------------------------------------------------------------------------------
fn extract_id(possible_id: Option<&str>) -> Option<i32> {
    match possible_id {
        Some(val) => {
            match val.parse::<i32>() {
                Ok(id) => Some(id),
                Err(parse_err) => {
                    eprintln!("Tried to parse {} but got error {}", val, parse_err);
                    None
                }
            }
        }
        None => None,
    }
}

fn extract_player_str(id: i32) -> String {
    match service::get_player_by_id(id) {
        Some(player) => parse_player(&player),
        None => String::from(""),
    }

}

fn parse_player(player: &Player) -> String {
    match serde_json::to_string(player) {
        Ok(player_str) => player_str,
        Err(err) => {
            eprintln!(
                "Serializing didn't work. Tried to serialized {:?} | error occurred {}",
                player,
                err
            );
            String::from("")
        }
    }
}
