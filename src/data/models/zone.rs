use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Zone<'a> {
    pub id: u32,
    // pub name: Option<&'a str>,
    pub name: Option<String>,
    #[serde(borrow)]
    pub raid_difficulty: Option<RaidDifficulty<'a>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RaidDifficulty<'a> {
    pub id: u8,
    pub name: &'a str,
}