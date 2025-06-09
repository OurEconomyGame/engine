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
}

pub struct Offer<'a, 'b> {
    pub entity: &'a mut Entity,
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
            OfferType::Sell => true, // Add inventory check logic later
        }
    }

    pub fn execute(&mut self) -> rusqlite::Result<()> {
        if !self.valid() {
            println!("Offer is not valid.");
            return Ok(());
        }

        let (target_offer_type, price_operator) = match self.offer_type {
            OfferType::Buy => (OfferType::Sell, "<="), // Buy offer: match Sell offers priced <= buy price
            OfferType::Sell => (OfferType::Buy, ">="), // Sell offer: match Buy offers priced >= sell price
        };

        // Dynamically build SQL based on operator
        let sql = format!(
            "SELECT id FROM extchange
     WHERE item = ?1
     AND type = ?2
     AND unit_price {} ?3",
            price_operator
        );

        let mut stmt = self.conn.prepare(&sql)?;

        let ids: Vec<i64> = stmt
            .query_map(
                params![
                    self.item.to_string(),
                    bool::from(target_offer_type), // Note: target_offer_type (opposite)
                    self.price
                ],
                |row| row.get(0),
            )?
            .filter_map(Result::ok)
            .collect();

        if !ids.is_empty() {
            println!("Matching offers found: {:?}", ids);
            // TODO: Do matching logic — transfer items/currency, delete matched offers, etc.
        } else {
            println!("No matching offers found. Saving to exchange.");
            self.save_to_db()?;
        }

        Ok(())
    }

    fn save_to_db(&self) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO extchange (item, type, amount, unit_price, unit_type, entity, entity_type)
             VALUES (?1, ?2, ?3, ?4, 'unit', ?5, ?6)",
            params![
                self.item.to_string(),
                bool::from(self.offer_type),
                self.quantity,
                self.price,
                self.entity.id(),
                self.entity.type_code()
            ],
        )?;

        println!("Offer saved to DB.");
        Ok(())
    }
}
