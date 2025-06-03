use crate::production_companies::*;
use crate::materials::Material::*;

pub fn tier_one_prod_list() -> [TierOneProdBase; 3] {
    [
        TierOneProdBase::new("Grain Company".to_string(), 100, Grain),
        TierOneProdBase::new("Power Plant".to_string(), 200, Electricity),
        TierOneProdBase::new("Water Company".to_string(), 1000, Water),
    ]
}

