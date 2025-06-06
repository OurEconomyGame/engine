use crate::{
    materials::Material, player::Player, production_companies::OwnsMaterials,
    recipies::ConsumableRecipe,
};
use json::{JsonValue, object};
use rusqlite::{Connection, Result, params};
use std::fmt;

#[derive(Debug, Clone)]
pub struct TierTwoProdBase {
    pub type_name: String,
    pub human_prod_rate: u32,
    pub creates: Material,
    pub consumes: ConsumableRecipe,
    pub cost: u32,
}

impl TierTwoProdBase {
    pub fn new(
        type_name: String,
        human_prod_rate: u32,
        creates: Material,
        consumes: ConsumableRecipe,
    ) -> Self {
        TierTwoProdBase {
            type_name,
            human_prod_rate,
            creates,
            consumes,
            cost: 400,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TierTwoProdInstance {
    pub id: Option<u32>,
    pub name: String,
    pub owner: u32,
    pub usd: u32,
    pub base_type: String,
    pub creates: Material,
    pub human_prod_rate: u32,
    pub human_workers: JsonValue,
    pub owns: OwnsMaterials,
    pub consumes: ConsumableRecipe,
}

impl TierTwoProdInstance {
    pub fn new(
        conn: &Connection,
        base: &TierTwoProdBase,
        name: String,
        owner: &mut Player,
    ) -> Result<Option<Self>, String> {
        if owner.usd < base.cost {
            return Ok(None);
        }
        owner.spend(base.cost);
        let mut instance = TierTwoProdInstance {
            id: None,
            name,
            owner: owner.id,
            usd: 0,
            base_type: base.type_name.clone(),
            creates: base.creates,
            human_prod_rate: base.human_prod_rate,
            human_workers: JsonValue::new_array(),
            owns: OwnsMaterials::new(),
            consumes: base.consumes.clone(),
        };
        instance
            .save(conn)
            .map_err(|e| format!("Failed to save instance: {}", e))?;
        owner.edit_shares(instance.id, 10000);
        Ok(Some(instance))
    }
    

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

                    for (mat, amount) in &self.consumes.inputs {
                        let owned = match mat {
                            Material::Grain => self.owns.grain,
                            Material::Electricity => self.owns.electricity,
                            Material::Water => self.owns.water,
                            Material::Food => {
                                return Err("Tier 2 companies shouldn’t consume Food!".to_string())
                            }
                        };
                        if owned < *amount {
                            return Err(format!(
                                "Not enough {:?} to produce {:?}",
                                mat, self.creates
                            ));
                        }
                    }

                    for (mat, amount) in &self.consumes.inputs {
                        match mat {
                            Material::Grain => self.owns.grain -= *amount,
                            Material::Electricity => self.owns.electricity -= *amount,
                            Material::Water => self.owns.water -= *amount,
                            Material::Food => {
                                return Err("Tier 2 companies shouldn’t consume Food!".to_string())
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

    pub fn save(&mut self, conn: &Connection) -> Result<u32> {
        let inputs_obj = object::Object::new();
        let inputs_obj = self
            .consumes
            .inputs
            .iter()
            .fold(inputs_obj, |mut obj, (mat, amt)| {
                obj.insert(mat.to_string_key(), JsonValue::from(*amt));
                obj
            });

        let data = object! {
            usd: self.usd,
            human_prod_rate: self.human_prod_rate,
            human_workers: self.human_workers.clone(),
            owns: {
                grain: self.owns.grain,
                electricity: self.owns.electricity,
                water: self.owns.water,
                food: self.owns.food,
            },
            creates: self.creates.to_string_key(),
            consumes: {
                inputs: inputs_obj,
            },
        };

        let data_str = data.dump();

        conn.execute(
            "INSERT INTO company (name, owner, type, data) VALUES (?1, ?2, ?3, ?4)",
            params![self.name, self.owner.to_string(), self.base_type, data_str],
        )?;
        self.id = Some(conn.last_insert_rowid() as u32);
        Ok(conn.last_insert_rowid() as u32)
    }

    pub fn earn(&mut self, money: u32) {
        self.usd += money;
    }
    pub fn spend(&mut self, amount: u32) {
        if amount > self.usd {
            eprintln!(
                "Warning: Tried to spend {} but only have {}",
                amount, self.usd
            );
            self.usd = 0;
        } else {
            self.usd -= amount;
        }
    }
    
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

            let owner = owner_str.parse::<u32>().unwrap_or(0);
            let usd = data_json["usd"].as_u32().unwrap_or(0);
            let human_prod_rate = data_json["human_prod_rate"].as_u32().unwrap_or(0);
            let human_workers = data_json["human_workers"].clone();

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

            let owns = OwnsMaterials {
                grain: data_json["owns"]["grain"].as_u32().unwrap_or(0),
                electricity: data_json["owns"]["electricity"].as_u32().unwrap_or(0),
                water: data_json["owns"]["water"].as_u32().unwrap_or(0),
                food: data_json["owns"]["food"].as_u32().unwrap_or(0),
            };

            Ok(Some(TierTwoProdInstance {
                id: Some(id),
                name,
                owner,
                usd,
                base_type,
                creates,
                human_prod_rate,
                human_workers,
                owns,
                consumes: ConsumableRecipe { inputs },
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for TierTwoProdBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tier Two Production Facility: {}", self.type_name)?;
        writeln!(f, "  Produces: {}", self.creates)?;
        writeln!(f, "  Human Production Rate: {}", self.human_prod_rate)?;
        writeln!(f, "  Consumes: {:?}", self.consumes.inputs)
    }
}

impl fmt::Display for TierTwoProdInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tier Two Production Facility: {}", self.name)?;
        writeln!(f, "  Type: {}", self.base_type)?;
        writeln!(f, "  Owned by: {}", self.owner)?;
        writeln!(f, "  Has: ${} USD", self.usd)?;
        writeln!(f, "  Produces: {}", self.creates)?;
        writeln!(f, "  Human Production Rate: {}", self.human_prod_rate)?;
        writeln!(f, "  Materials Owned: {:?}", self.owns)?;
        writeln!(f, "  Consumes: {:?}", self.consumes.inputs)?;
        writeln!(
            f,
            "  Current Human Workers JSON: {}",
            self.human_workers.dump()
        )
    }
}
