use std::rc::Rc;

use rustc_hash::FxHashMap;

use crate::{data::models::{passive_option::Utils, skill_buff::*}, RAW_SKILL_BUFF_MAP};

pub struct SkillBuffs<'a> {
    pub by_id: FxHashMap<u32, Rc<SkillBuff<'a>>>,
    pub by_name: FxHashMap<String, Vec<Rc<SkillBuff<'a>>>>,
}

impl<'a> SkillBuffs<'a> {
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
                    "Disrespect I" => EngravingType::Disrespect,
                    "Disrespect II" => EngravingType::Disrespect,
                    "Disrespect III" => EngravingType::Disrespect,
                    "Brawler" => EngravingType::MasterBrawler,
                    _ => EngravingType::None
                };

                if engraving_type != EngravingType::None {
                    let name = raw_skill_buff.name.unwrap().to_string();
                    let desc = raw_skill_buff.desc.clone().unwrap();
                    let skill_buff = Engraving {
                        id: *id,
                        engraving_type,
                        name: name.clone(),
                        desc: desc,
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
                let name = raw_skill_buff.name.unwrap_or("Unknown");
                let desc = raw_skill_buff.desc.clone();

                let skill_buff = Food {
                    id: *id,
                    name: name.to_string(),
                    desc,
                };

                let shared = Rc::new(SkillBuff::Food(skill_buff));
                by_id.insert(*id, shared.clone());
                by_name.entry(name.to_string()).or_default().push(shared);
                continue;
            }

            if raw_skill_buff.unique_group == SkillBuffUniqueGroup::DropsOfEther {
                let name = raw_skill_buff.name.unwrap().to_string();
                let desc = raw_skill_buff.desc.clone().unwrap();
                let skill_buff = Self::create_base_skill_buff(*id, name.clone(), desc);

                let shared = Rc::new(SkillBuff::DropsOfEther(skill_buff));
                by_id.insert(*id, shared.clone());
                by_name.entry(name).or_default().push(shared);
                continue;
            }

            if raw_skill_buff.unique_group == SkillBuffUniqueGroup::SupportEvolutionBuff {
                let name = raw_skill_buff.name.unwrap().to_string();
                let desc = raw_skill_buff.desc.clone().unwrap();
                let skill_buff = Self::create_base_skill_buff(*id, name.clone(), desc);

                let shared = Rc::new(SkillBuff::SupportEvolutionBuff(skill_buff));
                by_id.insert(*id, shared.clone());
                by_name.entry(name).or_default().push(shared);
                continue;
            }

            if raw_skill_buff.unique_group == SkillBuffUniqueGroup::SupportHyperAwakeningTechniqueBuff {
                let name = raw_skill_buff.name.unwrap().to_string();
                let desc = raw_skill_buff.desc.clone().unwrap();
                let skill_buff = Self::create_base_skill_buff(*id, name.clone(), desc);

                let shared = Rc::new(SkillBuff::SupportHyperAwakeningTechniqueBuff(skill_buff));
                by_id.insert(*id, shared.clone());
                by_name.entry(name).or_default().push(shared);
                continue;
            }

            if raw_skill_buff.unique_group == SkillBuffUniqueGroup::SupportMarking {
                let name = raw_skill_buff.name.unwrap().to_string();
                let desc = raw_skill_buff.desc.clone().unwrap();
                let skill_buff = Self::create_base_skill_buff(*id, name.clone(), desc);

                let shared = Rc::new(SkillBuff::SupportMarking(skill_buff));
                by_id.insert(*id, shared.clone());
                by_name.entry(name).or_default().push(shared);
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
                let name = raw_skill_buff.name.unwrap().to_string();
                let desc = raw_skill_buff.desc.clone().unwrap();
                let skill_buff = Self::create_base_skill_buff(*id, name.clone(), desc);

                let shared = Rc::new(SkillBuff::SupportAttackPowerBuff(skill_buff));
                by_id.insert(*id, shared.clone());
                by_name.entry(name).or_default().push(shared);
                continue;
            }

            if raw_skill_buff.set_name != SkillBuffSetName::NotApplicable {
                let name = raw_skill_buff.name.map(|str| str.to_string());
                let desc = raw_skill_buff.desc.clone();

                let skill_buff = Set {
                    id: *id,
                    set_type: raw_skill_buff.set_name,
                    name: name.clone(),
                    desc,
                };

                let shared = Rc::new(SkillBuff::Set(skill_buff));
                by_id.insert(*id, shared.clone());

                if let Some(name) = name {
                    by_name.entry(name).or_default().push(shared);
                }
                
                continue;
            }


            by_id.insert(*id, Rc::new(SkillBuff::Unknown(raw_skill_buff)));
        }

        Self {
            by_id,
            by_name
        }
    }

    fn create_base_skill_buff(id: u32, name: String, desc: String) -> BaseSkillBuff {
        BaseSkillBuff {
            id: id,
            name: name.clone(),
            desc,
        }
    }
}

#[derive(Debug, Default)]
pub enum SkillBuff<'a> {
    #[default]
    None,
    Unknown(&'a RawSkillBuff<'a>),
    Food(Food),
    Engraving(Engraving),
    DropsOfEther(BaseSkillBuff),
    BattleItem(BattleItem),
    SupportEvolutionBuff(BaseSkillBuff),
    SupportHyperAwakeningTechniqueBuff(BaseSkillBuff),
    SupportAttackPowerBuff(BaseSkillBuff),
    SupportIdentityBuff(IdentityBuff),
    SupportMarking(BaseSkillBuff),
    Set(Set)
}

#[derive(Debug, Default)]
pub struct BaseSkillBuff {
    pub id: u32,
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Default)]
pub struct Food {
    pub id: u32,
    pub name: String,
    pub desc: Option<String>,
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
    MastersTenacity,
    Disrespect
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

#[derive(Debug, Default)]
pub struct Set {
    pub id: u32,
    pub set_type: SkillBuffSetName,
    pub name: Option<String>,
    pub desc: Option<String>,
}
