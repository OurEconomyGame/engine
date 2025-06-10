use crate::materials::*;
use crate::player::Player;
use json::{JsonValue, object};
use rusqlite::{Connection, Result, params};
use std::fmt;

/// Tracks material quantities owned by a company instance
#[derive(Debug, Clone)]
pub struct OwnsMaterials {
    pub grain: u32,
    pub electricity: u32,
    pub water: u32,
    pub food: u32,
}

impl OwnsMaterials {
    pub fn new() -> Self {
        OwnsMaterials {
            grain: 0,
            electricity: 0,
            water: 0,
            food: 0,
        }
    }

    pub fn add(&mut self, mat: Material, amount: u32) {
        match mat {
            Material::Grain => self.grain += amount,
            Material::Electricity => self.electricity += amount,
            Material::Water => self.water += amount,
            Material::Food => self.food += amount,
        }
    }
}

/// Base configuration for Tier 1 companies
#[derive(Debug, Clone)]
pub struct TierOneProdBase {
    pub type_name: String,
    pub human_prod_rate: u32,
    pub robot_prod_rate: u32,
    pub creates: Material,
    pub max_human_workers: u32,
    pub max_robot_workers: u32,
    pub cost: u32,
}

impl TierOneProdBase {
    pub fn new(type_name: String, human_prod_rate: u32, creates: Material) -> Self {
        TierOneProdBase {
            type_name,
            human_prod_rate,
            robot_prod_rate: human_prod_rate * 2,
            creates,
            max_human_workers: 10,
            max_robot_workers: 1,
            cost: 200,
        }
    }
}

/// Instance of a Tier 1 company
#[derive(Debug, Clone)]
pub struct TierOneProdInstance {
    pub id: Option<u32>,
    pub name: String,
    pub owner: u32,
    pub usd: f32,
    pub base_type: String,
    pub creates: Material,
    pub human_prod_rate: u32,
    pub robot_prod_rate: u32,
    pub max_human_workers: u32,
    pub max_robot_workers: u32,
    pub human_workers: JsonValue,
    pub robot_workers: JsonValue,
    pub owns: OwnsMaterials,
}

impl TierOneProdInstance {
    pub fn new(
        conn: &Connection,
        base: &TierOneProdBase,
        name: String,
        owner: &mut Player,
    ) -> Result<Option<Self>, String> {
        if owner.usd < base.cost {
            return Ok(None);
        }

        owner.spend(base.cost);
        let mut instance = TierOneProdInstance {
            id: None,
            name,
            owner: owner.id,
            usd: 0.0,
            base_type: base.type_name.clone(),
            creates: base.creates,
            human_prod_rate: base.human_prod_rate,
            robot_prod_rate: base.robot_prod_rate,
            max_human_workers: base.max_human_workers,
            max_robot_workers: base.max_robot_workers,
            human_workers: JsonValue::new_array(),
            robot_workers: JsonValue::new_array(),
            owns: OwnsMaterials::new(),
        };

        instance
            .save(conn)
            .map_err(|e| format!("Failed to save instance: {}", e))?;
        owner.edit_shares(instance.id, 10000);
        Ok(Some(instance))
    }
    pub fn earn(&mut self, money: f32) {
        self.usd += money;
    }
    pub fn spend(&mut self, amount: f32) {
        if amount > self.usd {
            eprintln!(
                "Warning: Tried to spend {} but only have {}",
                amount, self.usd
            );
            self.usd = 0.0;
        } else {
            self.usd -= amount;
        }
    }
    pub fn hire_worker(&mut self, player: &Player) -> Result<(), String> {
        let current_workers = self.human_workers.len();
        if current_workers >= self.max_human_workers as usize {
            return Err("No available slots to hire a new human worker.".to_string());
        }

        // Check if already hired
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
                    entry[1] = true.into();
                    self.owns.add(self.creates, self.human_prod_rate);
                    player.energy -= 4;
                    return Ok(());
                }
            }
        }
        Err(format!(
            "Player {} is not hired at this facility.",
            player.id
        ))
    }

    pub fn reset_workers(&mut self) {
        for entry in self.human_workers.members_mut() {
            entry[1] = false.into(); // Reset worked status to false
        }
    }
}

impl TierOneProdInstance {
    pub fn save(&mut self, conn: &Connection) -> Result<u32> {
        let data = object! {
            usd: self.usd,
            human_prod_rate: self.human_prod_rate,
            robot_prod_rate: self.robot_prod_rate,
            max_human_workers: self.max_human_workers,
            max_robot_workers: self.max_robot_workers,
            human_workers: self.human_workers.clone(),
            robot_workers: self.robot_workers.clone(),
            owns: {
                grain: self.owns.grain,
                electricity: self.owns.electricity,
                water: self.owns.water,
                food: self.owns.food,
            },
            creates: self.creates.to_string_key(),
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
            self.id = Some(conn.last_insert_rowid() as u32);
            Ok(conn.last_insert_rowid() as u32)
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
            let usd = data_json["usd"].as_f32().unwrap_or(0.0);
            let human_prod_rate = data_json["human_prod_rate"].as_u32().unwrap_or(0);
            let robot_prod_rate = data_json["robot_prod_rate"].as_u32().unwrap_or(0);
            let max_human_workers = data_json["max_human_workers"].as_u32().unwrap_or(0);
            let max_robot_workers = data_json["max_robot_workers"].as_u32().unwrap_or(0);
            let human_workers = data_json["human_workers"].clone();
            let robot_workers = data_json["robot_workers"].clone();

            let creates_str = data_json["creates"].as_str().unwrap_or("");
            let creates = Material::from_str(creates_str).ok_or_else(|| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(std::fmt::Error),
                )
            })?;

            let owns = OwnsMaterials {
                grain: data_json["owns"]["grain"].as_u32().unwrap_or(0),
                electricity: data_json["owns"]["electricity"].as_u32().unwrap_or(0),
                water: data_json["owns"]["water"].as_u32().unwrap_or(0),
                food: data_json["owns"]["food"].as_u32().unwrap_or(0),
            };

            Ok(Some(TierOneProdInstance {
                id: Some(id),
                name,
                owner,
                usd,
                base_type,
                creates,
                human_prod_rate,
                robot_prod_rate,
                max_human_workers,
                max_robot_workers,
                human_workers,
                robot_workers,
                owns,
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for TierOneProdInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tier One Production Facility: {}", self.name)?;
        writeln!(f, "  Type: {}", self.base_type)?;
        writeln!(f, "  Owned by: {}", self.owner)?;
        writeln!(f, "  Has: ${} USD", self.usd)?;
        writeln!(f, "  Produces: {}", self.creates)?;
        writeln!(f, "  Human Production Rate: {}", self.human_prod_rate)?;
        writeln!(f, "  Robot Production Rate: {}", self.robot_prod_rate)?;
        writeln!(f, "  Max Human Workers: {}", self.max_human_workers)?;
        writeln!(f, "  Materials Owned: {:?}", self.owns)?;
        writeln!(
            f,
            "  Current Human Workers JSON: {}",
            self.human_workers.dump()
        )?;
        writeln!(
            f,
            "  Current Robot Workers JSON: {}",
            self.robot_workers.dump()
        )
    }
}

impl fmt::Display for TierOneProdBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tier One Production Facility: {}", self.type_name)?;
        writeln!(f, "  Produces: {}", self.creates)?;
        writeln!(f, "  Human Production Rate: {}", self.human_prod_rate)?;
        writeln!(f, "  Robot Production Rate: {}", self.robot_prod_rate)?;
        writeln!(f, "  Max Human Workers: {}", self.max_human_workers)
    }
}
