mod companies;
mod player;
mod load_json;
use json::{JsonValue};
use companies::*;
use load_json::*;
// use player::*;

fn main() {
    let my_json: JsonValue = load_json_file("data/prod/base/grain.json");
    let base_grain: TierOneProd = TierOneProd::new_base(my_json);
    println!("{base_grain}");
}