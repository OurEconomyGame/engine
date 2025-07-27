use super::*;
use crate::{materials::*, production::ProdInstance};
use rusqlite::{Connection, OptionalExtension};
impl ProdInstance {
    pub fn sell_all(&mut self, conn: &Connection) -> rusqlite::Result<()> {
        let materials = [
            Material::Electricity,
            Material::Water,
            Material::Grain,
            Material::Food,
        ];

        for &item in &materials {
            let amount = match item {
                Material::Electricity => self.owns.electricity,
                Material::Water => self.owns.water,
                Material::Grain => self.owns.grain,
                Material::Food => self.owns.food,
            };

            if amount == 0 {
                continue; // nothing to sell
            }

            // Get highest buy price for the item
            let sql = "
                SELECT MAX(unit_price)
                FROM extchange
                WHERE item = ?1
                AND type = 1  -- Buy offers
            ";

            let max_price_opt: Option<f32> = conn
                .query_row(sql, rusqlite::params![item.to_string_key()], |row| {
                    row.get(0)
                })
                .optional()?;

            let price = match max_price_opt {
                Some(p) => p,
                None => {
                    println!("No buy offers found for {:?}, skipping.", item);
                    continue;
                }
            };

            let prod_id = self.id.expect("No id for ProdInstance");

            let mut offer = Offer {
                entity: EntityRef::Borrowed(self),
                conn,
                item,
                quantity: amount,
                price,
                offer_type: OfferType::Sell,
            };

            if offer.valid() {
                if let Err(e) = offer.execute() {
                    eprintln!(
                        "Failed to execute sell_all offer for ProdInstance {} item {:?}: {}",
                        prod_id, item, e
                    );
                } else {
                    println!(
                        "Created sell_all offer for ProdInstance {} item {:?} at price {}",
                        prod_id, item, price
                    );
                }
            } else {
                println!(
                    "sell_all offer not valid for ProdInstance {} item {:?}",
                    prod_id, item
                );
            }
        }

        let _ = self.save(conn);
        Ok(())
    }
}
