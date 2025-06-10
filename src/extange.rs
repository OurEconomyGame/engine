use crate::{
    manufacturing::TierTwoProdInstance, materials::Material,
    production_companies::TierOneProdInstance,
};
use rusqlite::{Connection, params};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OfferType {
    Buy,
    Sell,
}

impl From<OfferType> for bool {
    fn from(offer: OfferType) -> Self {
        offer == OfferType::Buy
    }
}

impl From<bool> for OfferType {
    fn from(value: bool) -> Self {
        if value {
            OfferType::Buy
        } else {
            OfferType::Sell
        }
    }
}

pub enum Entity {
    Tier1(TierOneProdInstance),
    Tier2(TierTwoProdInstance),
}

impl Entity {
    pub fn usd(&self) -> f32 {
        match self {
            Entity::Tier1(t1) => t1.usd,
            Entity::Tier2(t2) => t2.usd,
        }
    }

    pub fn save(&mut self, conn:&Connection ) -> Result<u32, rusqlite::Error> {
        match self {
            Entity::Tier1(t1) => t1.save(conn),
            Entity::Tier2(t2) => t2.save(conn),
        }
    }

    pub fn id(&self) -> u32 {
        match self {
            Entity::Tier1(t1) => t1.id.expect("Tier1 entity must have an ID"),
            Entity::Tier2(t2) => t2.id.expect("Tier2 entity must have an ID"),
        }
    }

    pub fn type_code(&self) -> i32 {
        match self {
            Entity::Tier1(_) => 1,
            Entity::Tier2(_) => 2,
        }
    }

    pub fn earn(&mut self, amount: f32) {
        match self {
            Entity::Tier1(t1) => {
                t1.earn(amount);
            }
            Entity::Tier2(t2) => {
                t2.earn(amount);
            }
        }
    }

    pub fn add_material(&mut self, item: Material, quantity: u32) {
        match self {
            Entity::Tier1(t1) => {
                t1.owns.add(item, quantity);
            }
            Entity::Tier2(t2) => {
                t2.owns.add(item, quantity);
            }
        }
    }

}

/// EntityRef can either borrow a mutable Entity or own one
pub enum EntityRef<'a> {
    Borrowed(&'a mut Entity),
    Owned(Entity),
}

impl<'a> EntityRef<'a> {
    pub fn usd(&self) -> f32 {
        match self {
            EntityRef::Borrowed(e) => e.usd(),
            EntityRef::Owned(e) => e.usd(),
        }
    }

    pub fn id(&self) -> u32 {
        match self {
            EntityRef::Borrowed(e) => e.id(),
            EntityRef::Owned(e) => e.id(),
        }
    }

    pub fn type_code(&self) -> i32 {
        match self {
            EntityRef::Borrowed(e) => e.type_code(),
            EntityRef::Owned(e) => e.type_code(),
        }
    }

    /// Get mutable reference to inner Entity for mutation
    pub fn as_mut(&mut self) -> &mut Entity {
        match self {
            EntityRef::Borrowed(e) => *e,
            EntityRef::Owned(e) => e,
        }
    }
}

pub struct Offer<'a, 'b> {
    pub entity: EntityRef<'a>,
    pub conn: &'b Connection,
    pub item: Material,
    pub quantity: u32,
    pub price: f32,
    pub offer_type: OfferType,
}

impl<'a, 'b> Offer<'a, 'b> {
    pub fn valid(&self) -> bool {
        match self.offer_type {
            OfferType::Buy => self.entity.usd() >= self.quantity as f32 * self.price,
            OfferType::Sell => true, // Inventory check logic can be added later
        }
    }

    fn save_to_db(&self) -> rusqlite::Result<()> {
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

        println!("Offer saved to DB.");
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

impl<'a, 'b> Offer<'a, 'b> {
    pub fn execute(&mut self) -> rusqlite::Result<()> {
        println!("🔧 Starting offer execution.");
        println!(
            "👉 Offer details: item = {:?}, type = {:?}, qty = {}, price = {}",
            self.item, self.offer_type, self.quantity, self.price
        );
        println!(
            "👉 Entity ID: {}, type_code: {}, USD: {}",
            self.entity.id(),
            self.entity.type_code(),
            self.entity.usd()
        );

        if !self.valid() {
            println!("❌ Offer is not valid.");
            return Ok(());
        }

        let (target_offer_type, price_operator) = match self.offer_type {
            OfferType::Buy => (OfferType::Sell, "<="),
            OfferType::Sell => (OfferType::Buy, ">="),
        };

        println!(
            "🔎 Looking for matching {:?} offers with price {} {}",
            target_offer_type, price_operator, self.price
        );

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

        println!("📄 Final SQL query: {}", sql);

        let mut stmt = self.conn.prepare(&sql)?;
        println!("✅ SQL prepared successfully.");

        let mut rows = stmt.query(params![
            self.item.to_string_key(),
            bool::from(target_offer_type),
            self.price,
        ])?;
        println!(
            "🔍 Query executed with item='{}', type={}, price={}",
            self.item.to_string(),
            bool::from(target_offer_type),
            self.price
        );

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
            let matched_qty: u32 = row.get(1)?;
            let matched_price: f32 = row.get(2)?;
            let matched_entity_id: u32 = row.get(3)?;
            let matched_entity_type: i32 = row.get(4)?;

            println!(
                "➡️ Matched offer: ID = {}, Qty = {}, Price = {}, Entity ID = {}, Type = {}",
                matched_id, matched_qty, matched_price, matched_entity_id, matched_entity_type
            );

            let mut matched_offer =
                Offer::load_from_id(self.conn, matched_id)?.expect("Failed to load matched offer");

            let trade_qty = remaining_qty.min(matched_offer.quantity);
            println!("🔁 Trading {} units @ {}", trade_qty, matched_price);

            match self.offer_type {
                OfferType::Buy => {
                    println!("💰 Buyer (self) gains materials, seller (matched) gains USD.");
                    self.entity.as_mut().add_material(self.item, trade_qty);
                    matched_offer
                        .entity
                        .as_mut()
                        .earn(trade_qty as f32 * matched_price);
                }
                OfferType::Sell => {
                    println!("📦 Seller (self) gains USD, buyer (matched) gains materials.");
                    matched_offer
                        .entity
                        .as_mut()
                        .add_material(self.item, trade_qty);
                    self.entity.as_mut().earn(trade_qty as f32 * matched_price);
                }
            }

            remaining_qty -= trade_qty;
            matched_offer.quantity -= trade_qty;
            println!(
                "📉 Remaining qty: {}, Matched offer new qty: {}",
                remaining_qty, matched_offer.quantity
            );

            if matched_offer.quantity == 0 {
                println!("🗑️ Matched offer fully filled. Deleting from DB.");
                self.conn
                    .execute("DELETE FROM extchange WHERE id = ?1", params![matched_id])?;
            } else {
                println!("✏️ Updating matched offer with new quantity.");
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
