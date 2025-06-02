pub struct Player {
    pub id: u32,
    pub name: String,
}

impl Player {
    pub fn blank() -> Self {
        Player {
            id: 0,
            name: "Admin".to_string(),
        }
    }
}