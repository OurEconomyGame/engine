#![allow(dead_code)]

mod production_companies;
mod company_data;
mod materials;
mod player;
mod db;
mod own_struct;
mod recipies;
mod manufacturing;
use db::*;
use production_companies::*;
use company_data::*;
use player::*;
use rusqlite::Connection;

use crate::manufacturing::TierTwoProdInstance;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all_prods = tier_one_prod_list();
    let all_manu = tier_two_prod_list();
    let conn: Connection = init_db()?;
    let mut instances_id_one: Vec<u32> = Vec::new();
    let mut instances_id_two: Vec<u32> = Vec::new();
    let mut player = Player::new("Admin".to_string());
    player.earn(300_000);

    // Create Tier One instances
    for prod in &all_prods {
        match TierOneProdInstance::new(&conn, prod, "Something".to_string(), &mut player) {
            Ok(Some(mut instance)) => {
                if let Err(e) = instance.save(&conn).map(|id| instances_id_one.push(id)) {
                    eprintln!("Failed to save TierOneProdInstance: {}", e);
                }
            }
            Ok(None) => {
                println!("Not enough funds to create {}", prod.type_name);
            }
            Err(e) => {
                eprintln!("Error creating TierOneProdInstance: {}", e);
            }
        }
        println!("{prod}");
    }

    // Create Tier Two instances
    for prod in &all_manu {
        match TierTwoProdInstance::new(&conn, prod, "Something".to_string(), &mut player) {
            Ok(Some(mut instance)) => {
                if let Err(e) = instance.save(&conn).map(|id| instances_id_two.push(id)) {
                    eprintln!("Failed to save TierTwoProdInstance: {}", e);
                }
            }
            Ok(None) => {
                println!("Not enough funds to create {}", prod.type_name);
            }
            Err(e) => {
                eprintln!("Error creating TierTwoProdInstance: {}", e);
            }
        }
        println!("{prod}");
    }

    // Operate on Tier One instances
    for prod_id in &instances_id_one {
        let load_result = TierOneProdInstance::load(&conn, *prod_id);
        let mut prod = match load_result {
            Ok(Some(prod)) => prod,
            Ok(None) => {
                eprintln!("TierOneProdInstance with id {} not found", prod_id);
                continue;
            }
            Err(e) => {
                eprintln!("Error loading TierOneProdInstance {}: {}", prod_id, e);
                continue;
            }
        };

        if let Err(e) = prod.hire_worker(&player) {
            eprintln!("Error hiring worker on TierOneProdInstance {}: {}", prod_id, e);
        }
        if let Err(e) = prod.human_worked(&mut player) {
            eprintln!("Error during work on TierOneProdInstance {}: {}", prod_id, e);
        }

        println!("{prod}");
    }

    // Operate on Tier Two instances
    for prod_id in &instances_id_two {
        let load_result = TierTwoProdInstance::load(&conn, *prod_id);
        let mut prod = match load_result {
            Ok(Some(prod)) => prod,
            Ok(None) => {
                eprintln!("TierTwoProdInstance with id {} not found", prod_id);
                continue;
            }
            Err(e) => {
                eprintln!("Error loading TierTwoProdInstance {}: {}", prod_id, e);
                continue;
            }
        };

        if let Err(e) = prod.hire_worker(&player) {
            eprintln!("Error hiring worker on TierTwoProdInstance {}: {}", prod_id, e);
        }
        if let Err(e) = prod.human_worked(&mut player) {
            eprintln!("Error during work on TierTwoProdInstance {}: {}", prod_id, e);
        }

        println!("{prod}");
    }

    Ok(())
}
