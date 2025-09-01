use rusqlite::{Connection, Result};

use crate::{
    db::init_db,
    player::Player,
    production::{ALL_PRODS, ProdInstance},
};

mod db;
mod extange;
mod macros;
mod materials;
mod player;
mod production;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OurEconomy engine test runner starting...");
    let conn: Connection = init_db().expect("Db didnt connect");
    let mut player: Player = Player::blank();
    player.earn(500_000);
    for prod_base in ALL_PRODS[..3].iter() {
        let mut prod: ProdInstance = ProdInstance::new(
            &conn,
            prod_base,
            "Admin Production Facility".to_string(),
            &mut player,
        )?
        .expect("Prod Creation Failed");

        prod.earn(100_000.0);

        let _ = prod.hire_worker(&player);

        prod.reset_workers();

        let _ = prod.human_worked(&mut player);

        prod.quick_sell(&conn, prod.creates, 0.1, 100);

        let _ = prod.save(&conn);
    }
    let mut food_prod: ProdInstance = ProdInstance::new(
        &conn,
        &ALL_PRODS[3],
        "Admin Production Facility".to_string(),
        &mut player,
    )?
    .expect("Prod Creation Failed");

    food_prod.earn(100_000.0);

    let _ = food_prod.buy_needed(&conn, 5);
    let _ = food_prod.hire_worker(&player);

    food_prod.reset_workers();

    let _ = food_prod.human_worked(&mut player);
    let _ = food_prod.save(&conn);
    Ok(())
}
