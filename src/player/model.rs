
#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub gender: String,
    pub age: i32,
}

impl Player {
    pub fn new(id: i32, name: String, gender: String, age: i32) -> Player {
        Player {
            id,
            name,
            gender,
            age,
        }
    }
}
