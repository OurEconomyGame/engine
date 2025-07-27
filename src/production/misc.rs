use crate::{materials::Material, production::ProdInstance};

impl ProdInstance {
    pub fn earn(&mut self, money: f32) {
        self.usd += money;
    }
    pub fn spend(&mut self, amount: f32) {
        if amount > self.usd {
            eprintln!(
                "Warning: Tried to spend {} but only have {}",
                amount, self.usd
            );
            self.usd = 0.0;
        } else {
            self.usd -= amount;
        }
    }

    pub fn add_material(&mut self, item: Material, amount: u32) {
        self.owns.add(item, amount);
    }
}
