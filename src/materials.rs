// materials.rs

use std::fmt;

/// An enum representing various types of materials in the game.
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub enum Material {
    Grain,
    Electricity,
    Water,
    Food, // 👈 New material
}

impl Material {
    pub fn unit(&self) -> &'static str {
        match self {
            Material::Grain => "kg",
            Material::Electricity => "kWh",
            Material::Water => "liters",
            Material::Food => "packages", // 👈 New unit
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Material::Grain => "Grain",
            Material::Electricity => "Electricity",
            Material::Water => "Water",
            Material::Food => "Food",
        }
    }

    pub fn from_str(name: &str) -> Option<Material> {
        match name {
            "Grain" => Some(Material::Grain),
            "Electricity" => Some(Material::Electricity),
            "Water" => Some(Material::Water),
            "Food" => Some(Material::Food),
            _ => None,
        }
    }

    pub fn to_string_key(&self) -> &'static str {
        match self {
            Material::Grain => "Grain",
            Material::Electricity => "Electricity",
            Material::Water => "Water",
            Material::Food => "Food",
        }
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.display_name(), self.unit())
    }
}

