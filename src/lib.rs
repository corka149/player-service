extern crate iron;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod player;

use iron::prelude::*;
use router::Router;
use player::controller::*;

///
/// Bootstrap the service
///
pub fn run() {

    let mut router = Router::new();

    router.get("/player/:id", handler, "player");

    Iron::new(router).http("localhost:3000").unwrap();
}
