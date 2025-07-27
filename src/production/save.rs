use crate::production::ProdInstance;
use json::{JsonValue, object};
use rusqlite::{Connection, Result, params};

impl ProdInstance {
    pub fn save(&mut self, conn: &Connection) -> Result<u32> {
        // Build the JSON object for `consumes.inputs`
        let inputs_obj =
            self.recipe
                .inputs
                .iter()
                .fold(object::Object::new(), |mut obj, (mat, amt)| {
                    obj.insert(mat.to_string_key(), JsonValue::from(*amt));
                    obj
                });

        // Full data payload
        let data = object! {
            usd: self.usd,
            human_prod_rate: self.human_prod_rate,
            max_human_workers: self.max_human_workers,
            human_workers: self.human_workers.clone(),
            owns: {
                grain: self.owns.grain,
                electricity: self.owns.electricity,
                water: self.owns.water,
                food: self.owns.food,
            },
            creates: self.creates.to_string_key(),
            recipe: {
                inputs: inputs_obj,
            },
        };

        let data_str = data.dump();

        if let Some(id) = self.id {
            // Update existing row
            conn.execute(
                "UPDATE company SET name = ?1, owner = ?2, type = ?3, data = ?4 WHERE id = ?5",
                params![
                    self.name,
                    self.owner.to_string(),
                    self.base_type,
                    data_str,
                    id
                ],
            )?;
            Ok(id)
        } else {
            // Insert new row
            conn.execute(
                "INSERT INTO company (name, owner, type, data) VALUES (?1, ?2, ?3, ?4)",
                params![self.name, self.owner.to_string(), self.base_type, data_str],
            )?;
            let new_id = conn.last_insert_rowid() as u32;
            self.id = Some(new_id);
            Ok(new_id)
        }
    }
}
