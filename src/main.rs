#![allow(dead_code)]

mod production;
mod db;
mod extange;
mod materials;
mod own_struct;
mod player;
use rusqlite::Connection;

use crate::{db::init_db, extange::{Entity, EntityRef, Offer, OfferType}, player::Player, production::{manufacturing::TierTwoProdInstance, production_companies::TierOneProdInstance}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn: Connection = init_db()?;
    let instances_id_one = [1,2,3];
    let instances_id_two = [6];
    let mut player = Player::new("Admin".to_string());
    player.earn(300_000);


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
            eprintln!(
                "Error hiring worker on TierOneProdInstance {}: {}",
                prod_id, e
            );
        }
        prod.reset_workers();
        if let Err(e) = prod.human_worked(&mut player) {
            eprintln!(
                "Error during work on TierOneProdInstance {}: {}",
                prod_id, e
            );
        }
        // Create a sell offer for 100 units at $0.10
        let mut offer = Offer {
            entity: EntityRef::Owned(Entity::Tier1(prod.clone())), // Clone the instance as Entity
            conn: &conn,
            item: prod.creates, // Replace with actual material you want to sell
            quantity: prod.human_prod_rate,
            price: 0.10,
            offer_type: OfferType::Sell,
        };

        if offer.valid() {
            if let Err(e) = offer.execute() {
                eprintln!(
                    "Failed to save offer for TierOneProdInstance {}: {}",
                    prod.id.unwrap(),
                    e
                );
            } else {
                println!(
                    "Created sell offer for TierOneProdInstance {}!",
                    prod.id.unwrap()
                );
            }
        } else {
            println!(
                "Offer not valid for TierOneProdInstance {}!",
                prod.id.unwrap()
            );
        }
        let _ = prod.save(&conn);
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
            eprintln!(
                "Error hiring worker on TierTwoProdInstance {}: {}",
                prod_id, e
            );
        }
        prod.reset_workers();
        
        if let Err(e) = prod.human_worked(&mut player) {
            eprintln!(
                "Error during work on TierTwoProdInstance {}: {}",
                prod.id.ok_or("Id not found")?, e
            );
        }
        let _ = prod.save(&conn);
    }

    Ok(())
}
