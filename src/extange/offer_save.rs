use super::offer::*;
use super::offer_helpers::*;
use crate::{
    materials::Material, production::manufacturing::TierTwoProdInstance,
    production::production_companies::TierOneProdInstance,
};
use rusqlite::{Connection, params};

impl<'a, 'b> Offer<'a, 'b> {
    pub(super) fn save_to_db(&self) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO extchange (item, type, amount, unit_price, unit_type, entity, entity_type)
             VALUES (?1, ?2, ?3, ?4, 'unit', ?5, ?6)",
            params![
                self.item.to_string_key(),
                bool::from(self.offer_type),
                self.quantity,
                self.price,
                self.entity.id(),
                self.entity.type_code(),
            ],
        )?;

        Ok(())
    }

    pub fn load_from_id(
        conn: &'b Connection,
        offer_id: i64,
    ) -> rusqlite::Result<Option<Offer<'static, 'b>>> {
        let mut stmt = conn.prepare(
            "SELECT item, type, amount, unit_price, entity, entity_type
             FROM extchange
             WHERE id = ?1",
        )?;

        let row_result = stmt.query_row(params![offer_id], |row| {
            let item_str: String = row.get(0)?;
            let offer_type_bool: bool = row.get(1)?;
            let quantity: u32 = row.get(2)?;
            let price: f32 = row.get(3)?;
            let entity_id: u32 = row.get(4)?;
            let entity_type: i32 = row.get(5)?;

            let item = Material::from_str(&item_str).expect("Invalid material name in DB");

            let entity = match entity_type {
                1 => {
                    let t1 = TierOneProdInstance::load(conn, entity_id)?
                        .expect("TierOneProdInstance not found");
                    Entity::Tier1(t1)
                }
                2 => {
                    let t2 = TierTwoProdInstance::load(conn, entity_id)?
                        .expect("TierTwoProdInstance not found");
                    Entity::Tier2(t2)
                }
                _ => panic!("Unknown entity_type in DB"),
            };

            Ok((
                item,
                quantity,
                price,
                OfferType::from(offer_type_bool),
                entity,
            ))
        });

        match row_result {
            Ok((item, quantity, price, offer_type, entity)) => {
                // Return an Offer owning its entity (no leak)
                Ok(Some(Offer {
                    conn,
                    entity: EntityRef::Owned(entity),
                    item,
                    quantity,
                    price,
                    offer_type,
                }))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
