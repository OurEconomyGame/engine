use super::offer_helpers::*;
use crate::materials::Material;
use rusqlite::Connection;
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
}
