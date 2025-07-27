use crate::{
    materials::{Material, Recipe},
    production::Prod,
};

pub static ALL_PRODS: &[Prod] = &[
    Prod::new("Water Company", 500, Material::Water, Recipe::empty(), 50),
    Prod::new(
        "Power Plant",
        200,
        Material::Electricity,
        Recipe::empty(),
        50,
    ),
    Prod::new("Grain Farm", 100, Material::Grain, Recipe::empty(), 50),
    Prod::new(
        "Food Processing Plant",
        5,
        Material::Food,
        Recipe::food(),
        500,
    ),
];
