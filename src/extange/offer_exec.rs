use super::offer::*;
use super::offer_helpers::*;
use rusqlite::params;

impl<'a, 'b> Offer<'a, 'b> {
    pub fn execute(&mut self) -> rusqlite::Result<()> {
        if !self.valid() {
            println!("❌ Offer is not valid.");
            return Ok(());
        }

        let (target_offer_type, price_operator) = match self.offer_type {
            OfferType::Buy => (OfferType::Sell, "<="),
            OfferType::Sell => (OfferType::Buy, ">="),
        };

        let sql = format!(
            "SELECT id, amount, unit_price, entity, entity_type
         FROM extchange
         WHERE item = ?1
         AND type = ?2
         AND unit_price {} ?3
         ORDER BY unit_price {}",
            price_operator,
            if self.offer_type == OfferType::Buy {
                "ASC"
            } else {
                "DESC"
            }
        );

        let mut stmt = self.conn.prepare(&sql)?;

        let mut rows = stmt.query(params![
            self.item.to_string_key(),
            bool::from(target_offer_type),
            self.price,
        ])?;

        let mut remaining_qty = self.quantity;
        let mut match_found = false;

        while let Some(row) = rows.next()? {
            match_found = true;
            println!("✅ Match found!");

            if remaining_qty == 0 {
                println!("✅ No remaining quantity to match. Exiting loop.");
                break;
            }

            let matched_id: i64 = row.get(0)?;
            let matched_price: f32 = row.get(2)?;

            let mut matched_offer =
                Offer::load_from_id(self.conn, matched_id)?.expect("Failed to load matched offer");

            let trade_qty = remaining_qty.min(matched_offer.quantity);
            println!("🔁 Trading {} units @ {}", trade_qty, matched_price);

            match self.offer_type {
                OfferType::Buy => {
                    self.entity.as_mut().add_material(self.item, trade_qty);
                    matched_offer
                        .entity
                        .as_mut()
                        .earn(trade_qty as f32 * matched_price);
                    let _ = matched_offer.entity.as_mut().save(self.conn);
                }
                OfferType::Sell => {
                    matched_offer
                        .entity
                        .as_mut()
                        .add_material(self.item, trade_qty);
                    let _ = matched_offer.entity.as_mut().save(self.conn);
                    self.entity.as_mut().earn(trade_qty as f32 * matched_price);
                }
            }

            remaining_qty -= trade_qty;
            matched_offer.quantity -= trade_qty;

            if matched_offer.quantity == 0 {
                self.conn
                    .execute("DELETE FROM extchange WHERE id = ?1", params![matched_id])?;
            } else {
                self.conn.execute(
                    "UPDATE extchange SET amount = ?1 WHERE id = ?2",
                    params![matched_offer.quantity, matched_id],
                )?;
            }

            self.quantity = remaining_qty;
        }

        if !match_found {
            println!("🚫 No matching offers found.");
        }

        if self.quantity > 0 {
            println!("📬 Offer partially (or not) filled. Saving remainder to DB.");
            self.save_to_db()?;
        } else {
            println!("✅ Offer fully executed and removed.");
        }
        let _ = self.entity.as_mut().save(self.conn);
        println!("🎉 Offer execution complete.");
        Ok(())
    }
}
