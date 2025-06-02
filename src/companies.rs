use json::{parse, JsonValue};
use crate::player::Player;
use std::fmt;

pub struct TierOneProd {
    name: String,
    owner: Player,
    human_prod_rate: u32,
    robot_prod_rate: u32,
    human_workers: JsonValue,
    robot_workers: JsonValue,
    creates: String,
    max_human_workers: u32,
    max_robot_workers: u32,
}

impl TierOneProd {
    pub fn new_base(data: JsonValue) -> Self {
        // Extract everything with hard failure if missing or wrong type
        let name: String = data["name"]
            .as_str()
            .expect("Missing or invalid 'name'")
            .to_string();

        let human_prod_rate: u32 = data["human_prod_rate"]
            .as_u32()
            .expect("Missing or invalid 'human_prod_rate'");

        let robot_prod_rate: u32 = data["robot_prod_rate"]
            .as_u32()
            .expect("Missing or invalid 'robot_prod_rate'");

        let creates: String = data["produces"]
            .as_str()
            .expect("Missing or invalid 'produces'")
            .to_string();

        let max_human_workers: u32 = data["max_human_workers"]
            .as_u32()
            .expect("Missing or invalid 'max_human_workers'");

        let max_robot_workers: u32 = data["max_robot_workers"]
            .as_u32()
            .expect("Missing or invalid 'max_robot_workers'");

        TierOneProd {
            name,
            owner: Player::blank(),
            human_prod_rate,
            robot_prod_rate,
            human_workers: JsonValue::new_array(),
            robot_workers: JsonValue::new_array(),
            creates,
            max_human_workers,
            max_robot_workers,
        }
    }
    pub fn new_instance(base: TierOneProd, name: String, owner: Player) -> Self {
        TierOneProd {
            name,
            owner,
            human_prod_rate: base.human_prod_rate,
            robot_prod_rate: base.robot_prod_rate,
            human_workers: base.human_workers,
            robot_workers: base.robot_workers,
            creates: base.creates,
            max_human_workers: base.max_human_workers,
            max_robot_workers: base.max_robot_workers,
        }
    }
}

impl fmt::Display for TierOneProd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tier One Production Facility: {}", self.name)?;
        writeln!(f, "  Owned by: {}", self.owner.name)?;
        writeln!(f, "  Produces: {}", self.creates)?;
        writeln!(f, "  Human Production Rate: {}", self.human_prod_rate)?;
        writeln!(f, "  Robot Production Rate: {}", self.robot_prod_rate)?;
        writeln!(f, "  Max Human Workers: {}", self.max_human_workers)?;
        writeln!(f, "  Max Robot Workers: {}", self.max_robot_workers)?;
        writeln!(f, "  Current Human Workers: {}", self.human_workers)?;
        writeln!(f, "  Current Robot Workers: {}", self.robot_workers)
    }
}
