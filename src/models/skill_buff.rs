use rustc_hash::FxHashMap;

use crate::RAW_SKILL_BUFF_MAP;

pub struct SkillBuffs {
    pub by_id: FxHashMap<u32, SkillBuff>,
}

impl SkillBuffs {
    pub fn new() -> Self {

        // TO-DO 
        // for (id, skill_buff) in RAW_SKILL_BUFF_MAP.iter() {


        // }

        Self {
            by_id: FxHashMap::default()
        }
    }
}

#[derive(Debug, Default)]
pub enum SkillBuff {
    #[default]
    None,
    Unknown,
    Food,
    Engraving,
    DropsOfEther,
    BattleItem(BattleItem),
    SupportEvolutionBuff,
    SupportHyperAwakeningTechniqueBuff,
    SupportAttackPowerBuff,
    SupportIdentityBuff,
    SupportMarking,
    Set
}

#[derive(Debug, Default)]
pub struct BattleItem {

}