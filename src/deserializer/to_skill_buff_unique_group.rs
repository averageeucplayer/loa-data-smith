use serde::{Deserialize, Deserializer};

use crate::data::models::skill_buff::SkillBuffUniqueGroup;

pub fn to_skill_buff_unique_group<'de, D>(deserializer: D) -> Result<SkillBuffUniqueGroup, D::Error>
where
    D: Deserializer<'de>,
{
    let value = u32::deserialize(deserializer)?;
    match value {
        210230 => Ok(SkillBuffUniqueGroup::SupportMarking),
        101105 => Ok(SkillBuffUniqueGroup::PaladinAttackPowerBuff),
        314004 => Ok(SkillBuffUniqueGroup::ArtistAttackPowerBuff),
        101204 => Ok(SkillBuffUniqueGroup::BardAttackPowerBuff),
        368000 => Ok(SkillBuffUniqueGroup::PaladinIdentityBuff),
        212305 => Ok(SkillBuffUniqueGroup::SupportHyperAwakeningTechniqueBuff),
        2000260 | 2000360 => Ok(SkillBuffUniqueGroup::SupportBuff),
        501 | 502 | 503 | 504 | 505 => Ok(SkillBuffUniqueGroup::DropsOfEther),
        _ => Ok(SkillBuffUniqueGroup::Unknown),
    }
}