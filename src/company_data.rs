use crate::production_companies::*;
use crate::materials::Material::*;
use crate::manufacturing::*;
use crate::recipies::ConsumableRecipe;

pub fn tier_one_prod_list() -> [TierOneProdBase; 3] {
    [
        TierOneProdBase::new("Grain Company".to_string(), 100, Grain),
        TierOneProdBase::new("Power Plant".to_string(), 200, Electricity),
        TierOneProdBase::new("Water Company".to_string(), 1000, Water),
    ]
}

pub fn tier_two_prod_list() -> [TierTwoProdBase; 1] {
    let food_recipe = ConsumableRecipe::food_recipe();
    [
        TierTwoProdBase::new("Food Factory".to_string(), 50, Food, food_recipe)
    ]
}