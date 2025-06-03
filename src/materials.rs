// materials.rs

use std::fmt;

/// An enum representing various types of materials in the game.
#[derive(Clone,Copy,Debug)]
pub enum Material {
    Grain,
    Electricity,
    Water,
}

impl Material {
    /// Returns the unit of measurement for each material.
    pub fn unit(&self) -> &'static str {
        match self {
            Material::Grain => "kg",
            Material::Electricity => "kWh",
            Material::Water => "liters",
        }
    }

    /// Returns the human-readable name for each material.
    pub fn display_name(&self) -> &'static str {
        match self {
            Material::Grain => "Grain",
            Material::Electricity => "Electricity",
            Material::Water => "Water",
        }
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.display_name(), self.unit())
    }
}
