use crate::{
    materials::{Inventory, Material, Recipe},
    production::ProdInstance,
};
use json::JsonValue;
use rusqlite::{Connection, Result, params};

impl ProdInstance {
    pub fn load(conn: &Connection, id: u32) -> Result<Option<Self>> {
        let mut stmt = conn.prepare("SELECT name, owner, type, data FROM company WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let name: String = row.get(0)?;
            let owner_str: String = row.get(1)?;
            let base_type: String = row.get(2)?;
            let data_str: String = row.get(3)?;

            let data_json = json::parse(&data_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    data_str.len(),
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

            let owner: u32 = owner_str.parse::<u32>().unwrap_or(0);
            let usd: f32 = data_json["usd"].as_f32().unwrap_or(0.0);
            let human_prod_rate: u32 = data_json["human_prod_rate"].as_u32().unwrap_or(0);
            let human_workers: JsonValue = data_json["human_workers"].clone();
            let max_human_workers: u32 = data_json["max_human_workers"].as_u32().unwrap_or(10);
            let creates_str = data_json["creates"].as_str().unwrap_or("");
            let creates = Material::from_str(creates_str).unwrap();

            let mut inputs = Vec::new();
            for (key, value) in data_json["consumes"]["inputs"].entries() {
                // Assuming Material::from_str returns Option or Result
                if let Some(mat) = Material::from_str(key) {
                    let amt = value.as_u32().unwrap_or(0);
                    inputs.push((mat, amt));
                } else {
                    // Handle unknown material key gracefully
                    eprintln!("Unknown material key: {}", key);
                }
            }

            let owns = Inventory {
                grain: data_json["owns"]["grain"].as_u32().unwrap_or(0),
                electricity: data_json["owns"]["electricity"].as_u32().unwrap_or(0),
                water: data_json["owns"]["water"].as_u32().unwrap_or(0),
                food: data_json["owns"]["food"].as_u32().unwrap_or(0),
            };

            Ok(Some(ProdInstance {
                id: Some(id),
                name,
                owner,
                usd,
                base_type,
                creates,
                human_prod_rate,
                human_workers,
                max_human_workers,
                owns,
                recipe: Recipe {
                    inputs: std::borrow::Cow::Owned(inputs),
                },
            }))
        } else {
            Ok(None)
        }
    }
}
