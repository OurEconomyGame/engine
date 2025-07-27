use super::*;
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

        let sql = build_sql_query(self.offer_type, price_operator);

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

            process_trade(self, &mut matched_offer, trade_qty, matched_price)?;

            remaining_qty -= trade_qty;
            matched_offer.quantity -= trade_qty;

            update_db_after_trade(self.conn, matched_id, &mut matched_offer)?;

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
