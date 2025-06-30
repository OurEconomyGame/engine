use crate::{
    materials::Material,
    production::{
        manufacturing::TierTwoProdInstance,
        production_companies::{OwnsMaterials, TierOneProdInstance},
    },
};
use rusqlite::Connection;

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

#[derive(Clone)]
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

    pub fn save(&mut self, conn: &Connection) -> Result<u32, rusqlite::Error> {
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

    pub fn materials(&mut self) -> OwnsMaterials {
        match self {
            Entity::Tier1(t1) => t1.owns.clone(),
            Entity::Tier2(t2) => t2.owns.clone(),
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
