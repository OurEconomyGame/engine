#[derive(Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub usd: u32,
    pub energy: u8,
    pub data: JsonValue,
}

impl Player {
    pub fn blank() -> Self {
        Player {
            id: 0,
            name: "0".to_string(),
            usd: 0,
            data: JsonValue::new_object(),
            energy: 50,
        }
    }
    pub fn new(username: String) -> Self {
        Player {
            id: 1,
            name: username,
            usd: 0,
            data: JsonValue::new_object(),
            energy: 50,
        }
    }
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

use std::fmt;

use json::{JsonValue, object};


impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Player {{ id: {}, name: {}, usd: {} }}",
            self.id, self.name, self.usd
        )
    }
}
