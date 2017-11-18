pub mod persistence;
pub mod model;
pub mod service;
pub mod controller;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}
