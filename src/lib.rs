#![allow(dead_code)]

mod production_companies;
mod player;
mod materials;
mod company_data;
mod db;
/* 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_grain(){
        let my_json: json::JsonValue = load_json::load_json_file("data/prod/base/grain.json");
        let base_grain = companies::TierOneProd::new_base(my_json);
    }
}

*/