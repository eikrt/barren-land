use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterStats {
    pub abilities: HashMap<String, String>,
    pub stats: HashMap<String, u8>,
    pub creature_type: String,
}
impl Default for CharacterStats{
    fn default() -> CharacterStats{
        CharacterStats{
            abilities: HashMap::from([
                ("1".to_string(),"".to_string()),
                ("2".to_string(),"".to_string()),
                ("3".to_string(),"".to_string()),
                ("4".to_string(),"".to_string()),
                ("5".to_string(),"".to_string()),
            ]),
            stats: HashMap::from([
                ("str".to_string(),5),
                ("agi".to_string(),5),
                ("int".to_string(),5),
            ]),
            creature_type: "".to_string(),
        }
    }
}
pub trait Gatherer {
    fn gatherer() -> CharacterStats;
}
impl Gatherer for CharacterStats {
    fn gatherer() -> CharacterStats{
        CharacterStats{
            abilities: HashMap::from([
                ("1".to_string(),"".to_string()),
                ("2".to_string(),"".to_string()),
                ("3".to_string(),"".to_string()),
                ("4".to_string(),"".to_string()),
                ("5".to_string(),"".to_string()),
            ]),
            stats: HashMap::from([
                ("str".to_string(),5),
                ("agi".to_string(),5),
                ("int".to_string(),5),
            ]),
            creature_type: "".to_string(),
        }
    }
}
