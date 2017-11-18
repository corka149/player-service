use super::Gender;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    id: usize,
    name: String,
    gender: Gender,
    age: usize,
}

impl Player {
    pub fn new(id: usize, name: String, gender: Gender, age: usize) -> Player {

        Player {
            id,
            name,
            gender,
            age,
        }
    }

    pub fn clone(&self) -> Player {
        Player {
            id: self.id,
            name: self.name.clone(),
            gender: match self.gender {
                Gender::Male => Gender::Male,
                Gender::Female => Gender::Female,
            },
            age: self.age,
        }
    }
}
