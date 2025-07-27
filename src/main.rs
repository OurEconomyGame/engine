use crate::db::init_db;

mod db;
mod extange;
mod macros;
mod materials;
mod player;
mod production;

fn main() {
    println!("OurEconomy engine test runner starting...");
    let conn = init_db().expect("Db didnt connect");
    // TODO: Add test invocations here
}
