use crate::{player::Player, production::ProdInstance};
use json::JsonValue;

impl ProdInstance {
    pub fn hire_worker(&mut self, player: &Player) -> Result<(), String> {
        for entry in self.human_workers.members() {
            if let Some(pid) = entry[0].as_u32() {
                if pid == player.id {
                    return Err(format!("Player {} is already hired here!", pid));
                }
            }
        }
        let new_entry = JsonValue::Array(vec![player.id.into(), false.into()]);
        self.human_workers
            .push(new_entry)
            .map_err(|e| format!("Failed to add worker: {}", e))?;
        Ok(())
    }

    pub fn reset_workers(&mut self) {
        for entry in self.human_workers.members_mut() {
            entry[1] = false.into();
        }
    }
}
