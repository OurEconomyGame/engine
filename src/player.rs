pub struct Player {
    id: u32,
}

impl Player {
    pub fn blank() -> Self {
        Player {
            id: 0
        }
    }
}