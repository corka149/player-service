// HTTP & routing
extern crate iron;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
// Database
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_codegen;

pub mod player;
pub mod database;

// Database - for Diesel
pub mod schema;

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
