extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Highscores {
    beginner: Option<u64>,
    intermediate: Option<u64>,
    expert: Option<u64>,
}

impl Highscores {
    pub fn new() -> Self {
        Highscores {
            beginner: Option::None,
            intermediate: Option::None,
            expert: Option::None,
        }
    }

    pub fn get_beginner(&self) -> Option<u64> {
        self.beginner
    }

    pub fn get_intermediate(&self) -> Option<u64> {
        self.intermediate
    }

    pub fn get_expert(&self) -> Option<u64> {
        self.expert
    }

    pub fn set_beginner(&mut self, value: u64) {
        self.beginner = Option::Some(value);
    }

    pub fn set_intermediate(&mut self, value: u64) {
        self.intermediate = Option::Some(value);
    }

    pub fn set_expert(&mut self, value: u64) {
        self.expert = Option::Some(value);
    }
}

// TODO implement proper error handling
pub fn save(highscores: &Highscores, path: &str) {
    let mut file = File::create(path).unwrap();
    let json = serde_json::to_string(highscores).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

// TODO implement proper error handling
pub fn load(path: &str) -> Highscores {
    let mut file = File::open(path).unwrap();
    let mut json = String::new();
    file.read_to_string(&mut json).unwrap();
    let hs: Highscores = serde_json::from_str(&json).unwrap();
    hs
}