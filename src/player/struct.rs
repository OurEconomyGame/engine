#[derive(Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub usd: u32,
    pub energy: u8,
    pub data: JsonValue,
}

impl Player {
    pub fn blank() -> Self {
        Player {
            id: 0,
            name: "0".to_string(),
            usd: 0,
            data: JsonValue::new_object(),
            energy: 50,
        }
    }
    pub fn new(username: String) -> Self {
        Player {
            id: 1,
            name: username,
            usd: 0,
            data: JsonValue::new_object(),
            energy: 50,
        }
    }
}

use std::fmt;

use json::JsonValue;

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Player {{ id: {}, name: {}, usd: {} }}",
            self.id, self.name, self.usd
        )
    }
}
