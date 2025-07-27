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
