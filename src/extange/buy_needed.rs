use crate::{materials::*, production::ProdInstance};
use rusqlite::{Connection, OptionalExtension, params};

impl ProdInstance {
    pub fn buy_needed(&mut self, conn: &Connection, units_worth_of: u32) {
        // Calculate total needed quantities for inputs
        for (mat, amt_per_unit) in self.recipe.clone().inputs.iter() {
            let needed_total = amt_per_unit.checked_mul(units_worth_of).unwrap_or(u32::MAX);

            // How much do we currently have?
            let current_owned = match mat {
                Material::Electricity => self.owns.electricity,
                Material::Water => self.owns.water,
                Material::Grain => self.owns.grain,
                Material::Food => self.owns.food,
            };

            // Calculate how much we still need to buy
            if needed_total <= current_owned {
                // We have enough, skip this material
                continue;
            }

            let need_to_buy = needed_total - current_owned;

            // Find minimum price for this material on the market
            let mut stmt = match conn.prepare(
                "SELECT unit_price FROM extchange WHERE item = ?1 AND type = 1 ORDER BY unit_price ASC LIMIT 1"
            ) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("DB prepare failed for min price lookup: {}", e);
                    continue;
                }
            };

            let min_price_res: rusqlite::Result<Option<f32>> = stmt
                .query_row(params![mat.to_string_key()], |row| row.get(0))
                .optional();

            let min_price = match min_price_res {
                Ok(Some(p)) => p,
                Ok(None) => {
                    eprintln!("No buy offers found for {:?}, skipping", mat);
                    continue;
                }
                Err(e) => {
                    eprintln!("Failed to query min price: {}", e);
                    continue;
                }
            };

            // Place a quick buy order for the needed amount at the min price
            self.quick_buy(conn, *mat, min_price, need_to_buy);
        }
    }
}
