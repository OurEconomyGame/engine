#![allow(dead_code)]

mod production_companies;
mod company_data;
mod materials;
mod player;
mod db;
mod own_struct;
mod recipies;
mod manufacturing;
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