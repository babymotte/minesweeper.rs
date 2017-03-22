extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct Highscores {
    beginner: Option<f64>,
    intermediate: Option<f64>,
    expert: Option<f64>,
}

impl Highscores {

    pub fn new() -> Self {
        Highscores {
            beginner: Option::None,
            intermediate: Option::None,
            expert: Option::None,
        }
    }

    pub fn get_beginner(&self) -> Option<f64> {
        self.beginner
    }

    pub fn get_intermediate(&self) -> Option<f64> {
        self.intermediate
    }

    pub fn get_expert(&self) -> Option<f64> {
        self.expert
    }

    pub fn set_beginner(&mut self, value: f64) {
        self.beginner = Option::Some(value);
    }

    pub fn set_intermediate(&mut self, value: f64) {
        self.intermediate = Option::Some(value);
    }

    pub fn set_expert(&mut self, value: f64) {
        self.expert = Option::Some(value);
    }
}