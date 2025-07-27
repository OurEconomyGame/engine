use crate::materials::{Inventory, Material, Recipe};
use crate::player::Player;
use json::JsonValue;
use rusqlite::Connection;
use std::fmt;
#[derive(Debug, Clone)]
pub struct Prod {
    pub type_name: &'static str,
    pub human_prod_rate: u32,
    pub creates: Material,
    pub max_human_workers: u32,
    pub cost: u32,
    pub recipe: Recipe<'static>,
}

impl Prod {
    pub const fn new(
        type_name: &'static str,
        human_prod_rate: u32,
        creates: Material,
        recipe: Recipe<'static>,
        cost: u32,
    ) -> Self {
        Self {
            type_name,
            human_prod_rate,
            creates,
            max_human_workers: 10,
            cost,
            recipe,
        }
    }
}

impl fmt::Display for Prod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Production Facility: {}", self.type_name)?;
        writeln!(f, "  Produces: {}", self.creates)?;
        writeln!(f, "  Human Production Rate: {}", self.human_prod_rate)?;
        writeln!(f, "  Max Human Workers: {}", self.max_human_workers)?;
        writeln!(f, "  Cost: ${}", self.cost)?;
        writeln!(f, "  Recipe:\n{}", self.recipe)
    }
}

/// Instance of a company
#[derive(Debug, Clone)]
pub struct ProdInstance {
    pub id: Option<u32>,
    pub name: String,
    pub owner: u32,
    pub usd: f32,
    pub base_type: String,
    pub creates: Material,
    pub recipe: Recipe<'static>,
    pub human_prod_rate: u32,
    pub max_human_workers: u32,
    pub human_workers: JsonValue,
    pub owns: Inventory,
}

impl ProdInstance {
    pub fn new(
        conn: &Connection,
        base: &Prod,
        name: String,
        owner: &mut Player,
    ) -> Result<Option<Self>, String> {
        if owner.usd < base.cost {
            return Ok(None);
        }
        owner.spend(base.cost);
        let mut instance = ProdInstance {
            id: None,
            name,
            owner: owner.id,
            usd: 0.0,
            base_type: base.type_name.to_owned(),
            creates: base.creates,
            human_prod_rate: base.human_prod_rate,
            human_workers: JsonValue::new_array(),
            owns: Inventory::new(),
            recipe: base.recipe.clone(),
            max_human_workers: base.max_human_workers,
        };
        instance
            .save(conn)
            .map_err(|e| format!("Failed to save instance: {}", e))?;
        owner.edit_shares(instance.id, 10000);
        Ok(Some(instance))
    }
}
