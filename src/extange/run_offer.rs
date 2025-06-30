use super::*;
use crate::materials::*;
use rusqlite::Connection;

pub fn sell_all(conn: &Connection, mut prod: Entity, item: Material, price: f32) {
    let materials = prod.materials(); // Get materials once
    let prod_id = prod.id();
    let mut offer = Offer {
        entity: EntityRef::Borrowed(&mut prod), // Use the original 'prod' here
        conn: &conn,
        item: item,
        quantity: match item {
            Material::Electricity => materials.electricity,
            Material::Water => materials.water,
            Material::Grain => materials.grain,
            Material::Food => materials.food,
        },
        price: price,
        offer_type: OfferType::Sell,
    };

    if offer.valid() {
        if let Err(e) = offer.execute() {
            eprintln!(
                "Failed to save offer for TierOneProdInstance {}: {}",
                prod_id, e
            );
        } else {
            println!("Created sell offer for TierOneProdInstance {}!", prod_id);
        }
    } else {
        println!("Offer not valid for TierOneProdInstance {}!", prod_id);
    }

    let _ = prod.save(&conn);
}
