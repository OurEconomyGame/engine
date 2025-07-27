use super::*;
use crate::{materials::*, production::ProdInstance};
use rusqlite::Connection;
impl ProdInstance {
    pub fn quick_sell(&mut self, conn: &Connection, item: Material, price: f32, amount: u32) {
        let prod_id = self
            .id
            .expect("Id is None! Can't sell from a non-existent entity!");

        let mut offer = Offer {
            entity: EntityRef::Borrowed(self),
            conn: &conn,
            item,
            quantity: amount,
            price,
            offer_type: OfferType::Sell,
        };

        if offer.valid() {
            if let Err(e) = offer.execute() {
                eprintln!(
                    "Failed to execute sell offer for ProdInstance {}: {}",
                    prod_id, e
                );
            } else {
                println!("Created sell offer for ProdInstance {}!", prod_id);
            }
        } else {
            println!("Sell offer not valid for ProdInstance {}!", prod_id);
        }

        let _ = self.save(&conn);
    }

    pub fn quick_buy(&mut self, conn: &Connection, item: Material, price: f32, amount: u32) {
        let prod_id = self.id.expect("A non existant entity cant buy wares!");
        let mut offer = Offer {
            entity: EntityRef::Borrowed(self), // Use the original 'prod' here
            conn: &conn,
            item: item,
            quantity: amount,
            price: price,
            offer_type: OfferType::Buy,
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

        let _ = self.save(&conn);
    }
}
