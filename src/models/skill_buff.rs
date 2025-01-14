use std::{os::raw, rc::Rc};

use rustc_hash::FxHashMap;

use crate::{data::models::{passive_option::Utils, skill_buff::*}, RAW_SKILL_BUFF_MAP};

pub struct SkillBuffs {
    pub by_id: FxHashMap<u32, Rc<SkillBuff>>,
    pub by_name: FxHashMap<String, Vec<Rc<SkillBuff>>>,
}

impl SkillBuffs {
    pub fn new() -> Self {

        let mut by_id = FxHashMap::default();
        let mut by_name: FxHashMap<String, Vec<Rc<SkillBuff>>> = FxHashMap::default();

        for (id, raw_skill_buff) in RAW_SKILL_BUFF_MAP.iter() {

            if let Some(name) = raw_skill_buff.name {

                let identity_buff = match name {
                    "Moonfall" => IdentityBuffType::Moonfall,
                    "Serenade of Courage" => IdentityBuffType::SerenadeOfCourage,
                    _ => IdentityBuffType::None
                };

                if raw_skill_buff.target == SkillBuffTarget::SelfParty
                    && identity_buff != IdentityBuffType::None {
                    let name = raw_skill_buff.name.unwrap().to_string();
                    let desc = raw_skill_buff.desc.clone().unwrap();
                    let bonus = (raw_skill_buff.passive_options.get_value() / 100) as u8;

                    let skill_buff = IdentityBuff {
                        id: *id,
                        buff_type: identity_buff,
                        name: name.clone(),
                        desc,
                        bonus,
                    };

                    let shared = Rc::new(SkillBuff::SupportIdentityBuff(skill_buff));
                    by_id.insert(*id, shared.clone());
                    by_name.entry(name).or_default().push(shared);
                    continue;
                }
                
                let engraving_type = match name {
                    "Stabilized Status" => EngravingType::StabilizedStatus,
                    "Stabilized Status I" => EngravingType::StabilizedStatus,
                    "Stabilized Status II" => EngravingType::StabilizedStatus,
                    "Stabilized Status III" => EngravingType::StabilizedStatus,
                    "Magick Stream" => EngravingType::MagicStream,
                    "Magick Stream I" => EngravingType::MagicStream,
                    "Magick Stream II" => EngravingType::MagicStream,
                    "Magick Stream III" => EngravingType::MagicStream,
                    "Master's Tenacity I" => EngravingType::MastersTenacity,
                    "Master's Tenacity II" => EngravingType::MastersTenacity,
                    "Master's Tenacity III" => EngravingType::MastersTenacity,
                    "Master's Tenacity" => EngravingType::MastersTenacity,
                    "Ambush Master I" => EngravingType::AmbushMaster,
                    "Ambush Master II" => EngravingType::AmbushMaster,
                    "Ambush Master III" => EngravingType::AmbushMaster,
                    "Brawler" => EngravingType::MasterBrawler,
                    _ => EngravingType::None
                };

                if engraving_type != EngravingType::None {
                    let name = raw_skill_buff.name.unwrap().to_string();
                    let skill_buff = Engraving {
                        id: *id,
                        engraving_type,
                        name: raw_skill_buff.name.unwrap().to_string(),
                        desc: raw_skill_buff.desc.clone().unwrap(),
                        icon: raw_skill_buff.icon.unwrap().to_string()
                    };

                    let shared = Rc::new(SkillBuff::Engraving(skill_buff));
                    by_id.insert(*id, shared.clone());
                    by_name.entry(name).or_default().push(shared);
                    continue;
                }
                
            }

            if raw_skill_buff.buff_category == SkillBuffCategory::BattleItem {
                let name = raw_skill_buff.name.unwrap_or("Unknown").to_string();
                let skill_buff = BattleItem {
                    id: *id,
                    name: name.clone(),
                    desc: raw_skill_buff.desc.clone(),
                    icon: raw_skill_buff.icon.unwrap().to_string()
                };

                let shared = Rc::new(SkillBuff::BattleItem(skill_buff));
                by_id.insert(*id, shared.clone());
                by_name.entry(name).or_default().push(shared);
                continue;
            }

            if raw_skill_buff.buff_category == SkillBuffCategory::Cook {
                by_id.insert(*id, Rc::new(SkillBuff::Food(*id)));
                continue;
            }

            if raw_skill_buff.unique_group == SkillBuffUniqueGroup::DropsOfEther {
                by_id.insert(*id, Rc::new(SkillBuff::DropsOfEther(*id)));
                continue;
            }

            if raw_skill_buff.unique_group == SkillBuffUniqueGroup::SupportEvolutionBuff {
                by_id.insert(*id, Rc::new(SkillBuff::SupportEvolutionBuff(*id)));
                continue;
            }

            if raw_skill_buff.unique_group == SkillBuffUniqueGroup::SupportHyperAwakeningTechniqueBuff {
                by_id.insert(*id, Rc::new(SkillBuff::SupportHyperAwakeningTechniqueBuff(*id)));
                continue;
            }

            if raw_skill_buff.unique_group == SkillBuffUniqueGroup::PaladinIdentityBuff {
                let name = raw_skill_buff.name.unwrap().to_string();
                let skill_buff = IdentityBuff {
                    id: *id,
                    buff_type: IdentityBuffType::HolyAura,
                    name: name.clone(),
                    desc: raw_skill_buff.desc.clone().unwrap(),
                    bonus: 10
                };

                let shared = Rc::new(SkillBuff::SupportIdentityBuff(skill_buff));
                by_id.insert(*id, shared.clone());
                by_name.entry(name).or_default().push(shared);
                continue;
            }

            if raw_skill_buff.unique_group == SkillBuffUniqueGroup::ArtistAttackPowerBuff
                || raw_skill_buff.unique_group == SkillBuffUniqueGroup::BardAttackPowerBuff
                    || raw_skill_buff.unique_group == SkillBuffUniqueGroup::PaladinAttackPowerBuff {
                by_id.insert(*id, Rc::new(SkillBuff::SupportAttackPowerBuff(*id)));
                continue;
            }

            by_id.insert(*id, Rc::new(SkillBuff::Unknown(*id)));
        }

        Self {
            by_id,
            by_name
        }
    }
}

#[derive(Debug, Default)]
pub enum SkillBuff {
    #[default]
    None,
    Unknown(u32),
    Food(u32),
    Engraving(Engraving),
    DropsOfEther(u32),
    BattleItem(BattleItem),
    SupportEvolutionBuff(u32),
    SupportHyperAwakeningTechniqueBuff(u32),
    SupportAttackPowerBuff(u32),
    SupportIdentityBuff(IdentityBuff),
    SupportMarking(u32),
    Set(u32)
}

#[derive(Debug, Default)]
pub struct Engraving {
    pub id: u32,
    pub engraving_type: EngravingType,
    pub name: String,
    pub desc: String,
    pub icon: String
}

#[derive(Debug, Default)]
pub struct BattleItem {
    pub id: u32,
    pub name: String,
    pub desc: Option<String>,
    pub icon: String
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum EngravingType {
    #[default]
    None,
    StabilizedStatus,
    MasterBrawler,
    AmbushMaster,
    MagicStream,
    MastersTenacity
}

#[derive(Debug, Default)]
pub struct IdentityBuff {
    pub id: u32,
    pub buff_type: IdentityBuffType,
    pub name: String,
    pub desc: String,
    pub bonus: u8
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum IdentityBuffType {
    #[default]
    None,
    Moonfall,
    SerenadeOfCourage,
    HolyAura
}