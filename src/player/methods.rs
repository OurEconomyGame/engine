use super::Player;
use json::{JsonValue, object};

impl Player {
    pub fn earn(&mut self, money: u32) {
        self.usd += money;
    }
    pub fn spend(&mut self, amount: u32) {
        if amount > self.usd {
            eprintln!(
                "Warning: Tried to spend {} but only have {}",
                amount, self.usd
            );
            self.usd = 0;
        } else {
            self.usd -= amount;
        }
    }
    pub fn edit_shares(&mut self, company_id_option: Option<u32>, amount: i16) {
        let company_id = match company_id_option {
            Some(id) => id,
            None => panic!("Company ID is None — invalid input"),
        };

        // Ensure "owns" exists and is an object
        if self.data["owns"].is_null() {
            self.data["owns"] = object! {};
        }

        // Ensure "shares" exists and is an array
        if self.data["owns"]["shares"].is_null() {
            self.data["owns"]["shares"] = JsonValue::new_array();
        }

        // Now **search manually for company_id inside shares array**
        let shares_len = self.data["owns"]["shares"].len();
        let mut found_index = None;

        for i in 0..shares_len {
            if self.data["owns"]["shares"][i]["company_id"] == company_id {
                found_index = Some(i);
                break;
            }
        }

        if let Some(idx) = found_index {
            // Update existing share entry
            let current_amount = self.data["owns"]["shares"][idx]["amount"]
                .as_i16()
                .unwrap_or(0);
            let new_amount = current_amount + amount;

            if new_amount < 0 {
                panic!(
                    "Invalid operation: share amount would go negative ({} + {} = {})",
                    current_amount, amount, new_amount
                );
            }

            self.data["owns"]["shares"][idx]["amount"] = new_amount.into();
        } else {
            // Add new share entry
            if amount < 0 {
                panic!(
                    "Invalid operation: cannot create a share record with negative amount: {}",
                    amount
                );
            }

            self.data["owns"]["shares"]
                .push(object! {
                    company_id: company_id,
                    amount: amount
                })
                .unwrap();
        }
    }
}
