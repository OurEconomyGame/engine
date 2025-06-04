#![allow(dead_code)]

mod production_companies;
mod company_data;
mod materials;
mod player;
mod db;
mod own_struct;
use db::*;
use production_companies::*;
use company_data::*;
use player::*;
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all_prods = tier_one_prod_list();
    let conn: Connection = init_db().expect("Failed to initialize database");
    let mut instances_id: Vec<u32> = Vec::new();
    let mut player = Player::new("Admin".to_string());
    player.earn(300000);
    // Now you have a [TierOneProd]
    for prod in &all_prods {
        if let Some(mut instance) = TierOneProdInstance::new(&conn, prod, "Something".to_string(), &mut player) {
            let id = instance.save(&conn)?;
            instances_id.push(id);
        }
        println!("{prod}");
    }
    

    for prod_id in &mut instances_id {
        let mut prod = TierOneProdInstance::load(&conn, *prod_id).expect("Something went wrong with db").expect("Something else went wrong");
        prod.hire_worker(&player);
        prod.human_worked(&player);
        println!("{prod}");
    }
    Ok(())
}
