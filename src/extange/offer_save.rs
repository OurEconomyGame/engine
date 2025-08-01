use crate::{
    extange::{EntityRef, OfferType},
    materials::Material,
    production::ProdInstance,
};

use super::Offer;
use rusqlite::{Connection, params};

impl<'a, 'b> Offer<'a, 'b> {
    pub(super) fn save_to_db(&self) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO extchange (item, type, amount, unit_price, entity)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                self.item.to_string_key(),
                bool::from(self.offer_type),
                self.quantity,
                self.price,
                self.entity
                    .as_ref()
                    .id
                    .expect("Entity Id in saving extange offer is None!")
            ],
        )?;

        Ok(())
    }

    pub fn load_from_id(
        conn: &'b Connection,
        offer_id: i64,
    ) -> rusqlite::Result<Option<Offer<'static, 'b>>> {
        let mut stmt = conn.prepare(
            "SELECT item, type, amount, unit_price, entity
             FROM extchange
             WHERE id = ?1",
        )?;

        let row_result = stmt.query_row(params![offer_id], |row| {
            let item_str: String = row.get(0)?;
            let offer_type_bool: bool = row.get(1)?;
            let quantity: u32 = row.get(2)?;
            let price: f32 = row.get(3)?;
            let entity_id: u32 = row.get(4)?;

            let item = Material::from_str(&item_str).expect("Invalid material name in DB");

            let entity = ProdInstance::load(conn, entity_id)?.expect("Entity doesnt exist!!!");

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
