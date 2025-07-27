use crate::{materials::Material, player::Player, production::ProdInstance};

impl ProdInstance {
    pub fn human_worked(&mut self, player: &mut Player) -> Result<(), String> {
        for entry in self.human_workers.members_mut() {
            if let Some(pid) = entry[0].as_u32() {
                if pid == player.id {
                    if entry[1].as_bool().unwrap_or(false) {
                        return Err(format!("Player {} has already worked this cycle.", pid));
                    }
                    if player.energy < 4 {
                        return Err(format!("Player {} doesn't have enough energy.", pid));
                    }

                    for (mat, amount) in self.recipe.inputs.iter() {
                        let owned = match mat {
                            Material::Grain => self.owns.grain,
                            Material::Electricity => self.owns.electricity,
                            Material::Water => self.owns.water,
                            Material::Food => {
                                return Err("Tier 2 companies shouldn’t consume Food!".to_string());
                            }
                        };
                        if owned < *amount {
                            return Err(format!(
                                "Not enough {:?} to produce {:?}",
                                mat, self.creates
                            ));
                        }
                    }

                    for (mat, amount) in self.recipe.inputs.iter() {
                        match mat {
                            Material::Grain => self.owns.grain -= *amount,
                            Material::Electricity => self.owns.electricity -= *amount,
                            Material::Water => self.owns.water -= *amount,
                            Material::Food => {
                                return Err("Tier 2 companies shouldn’t consume Food!".to_string());
                            }
                        }
                    }

                    self.owns.add(self.creates, self.human_prod_rate);
                    player.energy -= 4;
                    entry[1] = true.into();
                    return Ok(());
                }
            }
        }
        Err(format!("Player {} is not hired here.", player.id))
    }
}
