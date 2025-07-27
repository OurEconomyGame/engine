use super::*;
use rusqlite::params;

// Helper to build SQL query string
pub fn build_sql_query(offer_type: OfferType, price_operator: &str) -> String {
    format!(
        "SELECT id, amount, unit_price, entity, entity_type
         FROM extchange
         WHERE item = ?1
         AND type = ?2
         AND unit_price {} ?3
         ORDER BY unit_price {}",
        price_operator,
        if offer_type == OfferType::Buy {
            "ASC"
        } else {
            "DESC"
        },
    )
}

// Helper to process a trade between offers
pub fn process_trade<'a, 'b>(
    offer: &mut Offer<'a, 'b>,
    matched_offer: &mut Offer<'a, 'b>,
    trade_qty: u32,
    matched_price: f32,
) -> rusqlite::Result<()> {
    match offer.offer_type {
        OfferType::Buy => {
            offer.entity.as_mut().add_material(offer.item, trade_qty);
            matched_offer
                .entity
                .as_mut()
                .earn(trade_qty as f32 * matched_price);
            let _ = matched_offer.entity.as_mut().save(offer.conn);
        }
        OfferType::Sell => {
            matched_offer
                .entity
                .as_mut()
                .add_material(offer.item, trade_qty);
            let _ = matched_offer.entity.as_mut().save(offer.conn);
            offer.entity.as_mut().earn(trade_qty as f32 * matched_price);
        }
    }
    Ok(())
}

// Helper to update database after trade
pub fn update_db_after_trade(
    conn: &rusqlite::Connection,
    matched_id: i64,
    matched_offer: &mut Offer<'_, '_>,
) -> rusqlite::Result<()> {
    if matched_offer.quantity == 0 {
        conn.execute("DELETE FROM extchange WHERE id = ?1", params![matched_id])?;
    } else {
        conn.execute(
            "UPDATE extchange SET amount = ?1 WHERE id = ?2",
            params![matched_offer.quantity, matched_id],
        )?;
    }
    Ok(())
}
