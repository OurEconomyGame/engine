#[derive(Debug,Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub usd: u32,
}

impl Player {
    pub fn blank() -> Self {
        Player {
            id: 0,
            name: "0".to_string(),
            usd: 0
        }
    }
    pub fn new(username: String) -> Self {
        Player {
            id: 1, 
            name: username,
            usd: 0,
        }
    }
    pub fn earn(&mut self, money: u32){
        self.usd += money;
    }
    pub fn spend(&mut self, money: u32){
        self.usd -= money;
    }
}

use std::fmt;

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Player {{ id: {}, name: {}, usd: {} }}",
            self.id, self.name, self.usd
        )
    }
}
