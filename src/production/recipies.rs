use crate::materials::Material;

#[derive(Debug, Clone)]
pub struct ConsumableRecipe {
    pub inputs: Vec<(Material, u32)>, // What materials, and how much
}

impl ConsumableRecipe {
    pub fn food_recipe() -> Self {
        Self {
            inputs: vec![
                (Material::Electricity, 10),
                (Material::Water, 5),
                (Material::Grain, 5),
            ],
        }
    }
}
