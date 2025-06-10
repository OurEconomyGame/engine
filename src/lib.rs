#![allow(dead_code)]


mod materials;
mod player;
mod db;
mod own_struct;
mod production;
mod extange;
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