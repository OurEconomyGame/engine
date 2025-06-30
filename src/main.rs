#![allow(dead_code)]

mod db;
mod extange;
mod materials;
mod own_struct;
mod player;
mod production;
use rusqlite::Connection;

use crate::{
    db::init_db,
    extange::{Entity, EntityRef, Offer, OfferType, run_offer},
    player::Player,
    production::{manufacturing::TierTwoProdInstance, production_companies::TierOneProdInstance},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn: Connection = init_db()?;
    let instances_id_one: [u32; 3] = [1, 2, 3];
    let instances_id_two: [u32; 1] = [4];
    let mut player: Player = Player::new("Admin".to_string());
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

        prod.reset_workers();
        if let Err(e) = prod.human_worked(&mut player) {
            eprintln!(
                "Error during work on TierOneProdInstance {}: {}",
                prod_id, e
            );
        }
        run_offer::sell_all(
            &conn,
            Entity::Tier1(prod.clone()),
            prod.creates.clone(),
            0.1,
        );
    }

    // Operate on Tier Two instances
    for prod_id in &instances_id_two {
        let load_result: Result<Option<TierTwoProdInstance>, rusqlite::Error> =
            TierTwoProdInstance::load(&conn, *prod_id);
        let mut prod: TierTwoProdInstance = match load_result {
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

        prod.reset_workers();

        if let Err(e) = prod.human_worked(&mut player) {
            eprintln!(
                "Error during work on TierTwoProdInstance {}: {}",
                prod.id.ok_or("Id not found")?,
                e
            );
        }
        let _ = prod.save(&conn);
    }

    Ok(())
}
