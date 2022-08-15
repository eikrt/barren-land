use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitStats{
    pub stats: HashMap<String, u8>,
}
impl Default for UnitStats{
    fn default() -> UnitStats{
        UnitStats{
            stats: HashMap::from([
                ("str".to_string(),5),
                ("agi".to_string(),5),
                ("int".to_string(),5),
            ]),
        }
    }
}
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
pub trait Kobold{
    fn kobold () -> CharacterStats;
}
impl Kobold for CharacterStats {
    fn kobold() -> CharacterStats{
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
pub trait Ogre{
    fn ogre () -> CharacterStats;
}
impl Ogre for CharacterStats {
    fn ogre() -> CharacterStats{
        CharacterStats{
            abilities: HashMap::from([
                ("1".to_string(),"Shout".to_string()),
                ("2".to_string(),"Roll Attack".to_string()),
                ("3".to_string(),"Charge".to_string()),
                ("4".to_string(),"Earth Spike".to_string()),
                ("5".to_string(),"Earthquake".to_string()),
            ]),
            stats: HashMap::from([
                ("str".to_string(),9),
                ("agi".to_string(),2),
                ("int".to_string(),3),
            ]),
            creature_type: "".to_string(),
        }
    }
}
pub trait Gnoll{
    fn gnoll() -> CharacterStats;
}
impl Gnoll for CharacterStats {
    fn gnoll() -> CharacterStats{
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
pub trait Goblin{
    fn goblin() -> CharacterStats;
}
impl Goblin for CharacterStats {
    fn goblin() -> CharacterStats{
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
pub trait Rat{
    fn rat() -> CharacterStats;
}
impl Rat for CharacterStats {
    fn rat() -> CharacterStats{
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
