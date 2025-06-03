#![allow(dead_code)]

mod production_companies;
mod company_data;
mod materials;
mod player;
mod db;
use production_companies::*;
use company_data::*;
use player::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all_prods = tier_one_prod_list();
    let mut instances: Vec<TierOneProdInstance> = Vec::new();
    let mut player = Player::new("Admin".to_string());
    player.earn(300000);
    // Now you have a [TierOneProd]
    for prod in &all_prods {
        if let Some(instance) = TierOneProdInstance::new(prod, "Something".to_string(), &mut player) {
            instances.push(instance);
        }
        println!("{prod}");
    }

    for prod in &mut instances {
        prod.hire_worker(&player);
        prod.human_worked(&player);
        println!("{prod}");
    }
    Ok(())
}
