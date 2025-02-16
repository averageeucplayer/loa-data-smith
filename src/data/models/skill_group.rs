use crate::deserializer::*;
use chrono::Duration;
use once_cell::sync::Lazy;
use rustc_hash::FxHashSet;
use serde::{Deserialize, Deserializer, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::deserializer::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter)]
#[repr(u32)]
pub enum SkillGroup1 {
    None,
    AllClassSkills(u32),
    AwakeningAndHyperAwakeningTechniqueSkills(u32),
    HyperAwakeningTechniqueSkills(u32),
    AwakeningSkills(u32),
    HyperAwakeningSkills(u32),
    ShadowhunterSkills(u32),
    StrikerSkills(u32),
    MachinistSkills(u32),
    ArtilleristSkills(u32),
    SummonerSkills(u32),
    GunlancerSkills(u32),
    AeromancerSkills(u32),
    ArcanistSkills(u32),
    BerserkerSkills(u32),
    SlayerSkills(u32),
    BreakerSkills(u32),
    SorceressSkills(u32),
    GunslingerSkills(u32),
    DeadeyeSkills(u32),
    SouleaterSkills(u32),
    SharpshooterSkills(u32),
    ReaperSkills(u32),
    SharpshooterSilverhawkSkills(u32),
    SupportIdentityAndHyperAwakeningTechniqueSkills = 16000001,
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter)]
#[repr(u32)]
pub enum SkillGroup {
    None,
    BreakerSkillAsuraDestructionBasicAttack = 36000,
    PaladinSkillExecutionOfJudgment = 2365600,
    PaladinSkillExecutionOfJudgment1 = 2365700,
    PaladinHolySkills = 140001,
    PaladinJudgmentSkills1 = 2360090,
    PaladinJudgmentSkills2 = 2360040,
    PaladinAllJudgmentSkills = 2360080,
    PaladinAllAwakeningSkills = 2365200,
    PaladinJudgementAwakeningSkills = 2365100,
    PaladinAwakeningSkills = 140003,
    PaladinAwakeningAndHyperAwakeningTechniqueSkills = 2365000,
    PaladinJudgementSkills = 140002,
    ArtistSkillPaintIllusionDoor = 610001,
    AllClassHandOrWeaponAttackSkills = 991000,
    WarriorMaleSkills = 400000010,
    SorceressHyperAwakeningSkills = 2375000,
    SorceressFireSkills = 240000,
    SorceressSkills1 = 240001,
    SorceressSkills2 = 2370400,
    Sorceress3Skills = 2370900, 
    SorceressLightningSkills = 240002,
    SorceressMiscSkills = 240003,
    SorceressAwakeningSkills = 240004,
    BardIdentitySkills1 = 24000,
    BardIdentitySkills2 = 29070,
    AllBardSkills = 220000,
    BardAwakeningSkills = 220003,
    BardHyperAwakeningSkills = 2215000,
    BardWindOfMusicSkill = 29071,
    BardIdentityAndHyperAwakeningTechniqueSkills = 220004,
    BardSerenadeOfCourageSkill = 2210200,
    Bard3Skills = 220001,
    BardHarpOfRhytmSkills = 211808,
    ArtistSkills = 610000,
    ArtistAwakeningSkills = 610002,
    ArtistIdentitySkills = 610003,
    ArtistHyperAwakeningSkills = 2315000,
    AllReaperSkills = 2260901,
    ReaperHyperAwakeningTechniqueSkills2 = 2265400,
    ReaperAllAwakeningSkills = 430003,
    ReaperHyperAwakeningTechniqueSkills = 2265500,
    ReaperHyperAwakeningTechniqueSkills1 = 2265300,
    ReaperAwakeningSkills = 2265000,
    ReaperSkills1 = 2260400,
    ReaperSkills2 = 430001,
    ReaperSkills3 = 430004,
    ReaperSkills4 = 2260200,
    ReaperSkills5 = 60000,
    ReaperSkills6 = 430002,
    ReaperSkills7 = 2260500,
    ReaperSkills8 = 2260000,
    ReaperSkills9 = 430000,
    ReaperSkills10 = 2260401,
    ReaperSkills311 = 2260900,
    AllReaperSkills2 = 203,
    ReaperFatalStepSkill = 430005,
    SouleaterAwakeningSkills1 = 2465000,
    SouleaterAwakeningSkills2 = 2465300,
    SouleaterAwakeningSkills3 = 440004,
    SouleaterHyperAwakeningSkills = 2465500,
    SouleaterSkills1 = 2460500,
    SouleaterSkills2 = 440001,
    SouleaterSkills3 = 2460901,
    SouleaterSkills4 = 2460400,
    SouleaterSkills5 = 440002,
    SouleaterSkills6 = 440003,
    SouleaterComboAttackSkill = 440000,
    SouleaterHyperAwakeningTechniqueSkills = 2465400,
    WardancerAwakeningSkills = 2225000,
    WardancerEsotericSkills = 2221101,
    WardancerEsotericSkills1 = 2221100,
    WardancerEsotericSkills2 = 2220400,
    WardancerSkills = 310000,
    WardancerSkills1 = 2220900,
    WardancerAwakeningSkills2 = 310003,
    DeadeyeAwakeningSkills = 2295200,
    Deadeye2Skills = 2295300,
    DeadeyeSkills1 = 520000,
    DeadeyeSkills2 = 2293400,
    DeadeyeSkills3 = 520001,
    DeadeyeSkills4 = 520002,
    DeadeyeSkills5 = 2292200,
    DeadeyeAwakeningSkills2 = 520003,
    DeadeyeAwakeningSkills1 = 2295100,
    Deadeye3Skills = 2295000,
    GlaivierSkills1 = 2340900,
    GlaivierSkills2 = 340001,
    GlaivierSkills3 = 2341000,
    GlaivierSkills4 = 340002,
    GlaivierFocusStanceSkill = 2340100,
    GlaivierFlurryStanceSkill = 2340101,
    GlaivierAwakeningSkills1 = 2345000,
    GlaivierAwakeningSkills2 = 340003,
    ArcanistSkills1 = 2192001,
    ArcanistSkills2 = 2192000,
    ArcanistSkills3 = 210001,
    ArcanistSkills4 = 2190402,
    ArcanistSkills5 = 210000,
    ArcanistSkills6 = 2190401,
    ArcanistSkills7 = 2190400,
    ArcanistSkills8 = 14000003,
    ArcanistSkills9 = 2190901,
    ArcanistSkills10 = 14000,
    ArcanistSkills11 = 2190902,
    ArcanistSkills12 = 210002,
    ArcanistSkills13 = 2190900,
    ArcanistAwakeningSkills1 = 210003,
    ArcanistAwakeningSkills2 = 2195000,
    SoulfistSkills1 = 2240900,
    SoulfistSkills2 = 2240800,
    SoulfistSkills3 = 330000,
    SoulfistAwakeningSkills2 = 2245000,
    SoulfistFlashStepSkill = 2241000,
    SoulfistAwakeningSkills = 330003,
    AllSlayerAwakeningSkills = 2455200,
    SlayerAwakeningAndHyperAwakeningTechniqueSkills = 2455000,
    SlayerSkills2 = 150000,
    SlayerSkills3 = 150004,
    SlayerAwakeningSkills = 150003,
    SlayerHyperAwakeningSkills = 2455100,
    ShadowhunterSkills1 = 2270400,
    ShadowhunterSkills2 = 420004,
    ShadowhunterSkills3 = 2271100,
    ShadowhunterSkills4 = 2270901,
    ShadowhunterSkills5 = 2270600,
    ShadowhunterSkills6 = 2270900,
    ShadowhunterSkills7 = 42001,
    ShadowhunterSkills8 = 2270500,
    ShadowhunterSkills9 = 420000,
    ShadowhunterSkills10 = 2271000,
    ShadowhunterSkills11 = 420001,
    ShadowhunterAwakeningSkills1 = 2275000,
    ShadowhunterAwakeningSkills2 = 420003,
    ShadowhunterAwakeningSkills3 = 42000,
    ShadowhunterHyperAwakeningTechniqueSkills = 2275500,
    ShadowhunterHyperAwakeningTechniqueSkills1 = 2275300,
    ShadowhunterHyperAwakeningTechniqueSkills2 = 2275400,
    ScrapperAwakeningSkills1 = 2235000,
    ScrapperAwakeningSkills2 = 320003,
    ScrapperTenacityReleaseSkill = 320004,
    ScrapperSkills1 = 320002,
    ScrapperSkills2 = 320001,
    GunlancerSkills1 = 2170110,
    GunlancerSkills2 = 2170070,
    GunlancerSkills3 = 130000,
    GunlancerSkills4 = 2170040,
    GunlancerSkills5 = 2170050,
    GunlancerSkills6 = 130001,
    GunlancerSkills7 = 2170090,
    GunlancerSkills8 = 2170080,
    GunlancerBlueSkills = 2170100,
    GunlancerAwakeningSkills = 2175200,
    GunlancerHyperAwakeningTechniqueSkills = 13000,
    GunlancerBattlefieldShieldSkill = 14001,
    GunlancerShieldDashAndNelasiasEnergySkills = 2170081,
    GunlancerAwakeningAndHyperAwakeningTechniqueSkills = 2175000,
    GunlancerAwakeningSkills1 = 130003,
    GunlancerAwakeningSkills2 = 2175100,
    GunlancerBattlefieldShieldSkill1 = 2170071,
    GunlancerBattlefieldShieldSkill2 = 77390000,
    DestroyerHyperAwakeningSkills = 120003,
    DestroyerAllAwakeningSkills1 = 2185000,
    DestroyerAllAwakeningSkills2 = 2185200,
    DestroyerSkills1 = 120001,
    DestroyerSkills2 = 120002,
    DestroyerSkills3 = 2180090,
    Destroyer2Skills1 = 29,
    Destroyer2Skills2 = 13001,
    DestroyerBasicAttackVortexGravitySkills1 = 120004,
    DestroyerBasicAttackVortexGravitySkills = 2160060,
    DestroyerHyperAwakeningSkills1 = 2185100,
    AeromancerSkills1 = 630102,
    AeromancerSkills2 = 2320900,
    AeromancerSkills3 = 2320401,
    AeromancerSkills4 = 2321100,
    AeromancerSkills5 = 620000,
    AeromancerSkills6 = 630101,
    AeromancerSkills7 = 31,
    AeromancerSkills8 = 2320901,
    AeromancerSunShowerSkills1 = 62000,
    AeromancerSunShowerSkills2 = 2320400,
    AeromancerSunShowerSkills3 = 2321101,
    AeromancerSkillSunShower = 2320700,
    AeromancerHyperAwakeningTechniqueSkills = 2325300,
    AeromancerAwakeningSkills1 = 620001,
    AeromancerHyperAwakeningSkills = 2325400,
    AeromancerAwakeningSkills = 2325000,
    AeromancerHyperAwakeningTechniqueSkills1 = 2325500,
    AeromancerSunShowerSkill = 620002,
    AllClassSkills = 16000000,
    AllClassSkills1 = 200,
    AllClassSkills2 = 36,
    AllClassSkills3 = 14000000,
    AllClassSkills4 = 10000002,
    AllClassSkills5 = 12000002,
    AllClassSkills6 = 14000001,
    AllClassSkills7 = 28,
    AllClassSkills8 = 15000000,
    AllClassSkills9 = 27,
    AllClassSkills10 = 17000000,
    AllClassSkills11 = 100,
    AllClassSkills12 = 10000001,
    AllClassSkills13 = 33,
    AllClassAwakeningSkills = 991021,
    AllClassAwakeningSkills1 = 10000000,
    AllClassAwakeningSkills2 = 30,
    AllClassAwakeningSkills3 = 11000002,
    AllClassAwakeningSkills4 = 9900,
    AllClassAwakeningSkills5 = 11000000,
    AllClassAwakeningSkills6 = 991011,
    AllClassAwakeningSkills7 = 991001,
    StrikerWardancerSouleaterSkills = 202,
    SoulfistMachinistScrapperBreakerSkills = 201,
    Class21Skills = 13000002,
    BerserkerHyperAwakeningSkills = 2165100,
    BerserkerAllAwakeningSkills = 2165200,
    BerserkerAwakeningSkills = 110003,
    BerserkerSkills1 = 110004,
    BerserkerSkills2 = 110000,
    BerserkerAllSkills = 2160080,
    BerserkerAwakeningAndHyperAwakeningTechniqueSkills = 2165000,
    Berserker2Skills = 1100000,
    BerserkerAwakeningSkills1 = 11000,
    BerserkerSlayerSkills = 11001,
    GunslingerSkills = 2512030,
    SummonerAwakeningSkills = 2205000,
    SummonerSkills = 23001,
    SummonerSkills1 = 23000,
    SummonerSkills2 = 2200900,
    SummonerSkills3 = 199,
    SummonerSkills4 = 230000,
    SummonerSkills5 = 2200800,
    SummonerSkills6 = 32,
    SummonerSkills7 = 29073,
    SummonerSkills8 = 29072,
    SummonerSkills9 = 2200700,
    SummonerSkills10 = 230004,
    SummonerSkills11 = 300,
    SummonerUnknownSkills1 = 2200910,
    SummonerUnknownSkills2 = 230005,
    SummonerUnknownSkills3 = 230006,
    SummonerOshSkill = 230011,
    SummonerPhoenixSkill = 230013,
    Summoner2Skills = 2201100,
    SummonerAwakeningSkills1 = 230003,
    SummonerJahiaLigheasSkill = 230014,
    SummonerAlimajiSkill = 230012,
    AllGunslingerSkills = 2512010,
    GunslingerSkills1 = 2294200,
    GunslingerSkills2 = 550000,
    GunslingerSkills3 = 550001,
    GunslingerSkills4 = 2512040,
    GunslingerSkills5 = 550002,
    GunslingerSkills6 = 2512020,
    GunslingerHyperAwakeningSkills = 2385100,
    GunslingerAwakeningSkills = 550003,
    GunslingerAllAwakeningSkills = 2385200,
    Gunslinger2Skills = 2512000,
    Gunslinger3Skills = 2385000,
    StrikerSkills1 = 310001,
    StrikerSkills2 = 2391005,
    StrikerSkills3 = 2390801,
    StrikerSkills4 = 2390900,
    StrikerSkills5 = 2391001,
    StrikerSkills6 = 350000,
    StrikerSkills7 = 2390800,
    StrikerSkills8 = 2390901,
    StrikerSkills9 = 2391003,
    StrikerSkills10 = 2391004,
    Striker4Skills = 2391000,
    Striker4Skills1 = 350001,
    StrikerAwakeningSkills = 350003,
    StrikerHyperAwakeningSkills = 2395000, 
    StrikerEsotericSkills = 2391002,
    StrikerSkillAdvancingStrike = 39370,
    BreakerAwakeningSkills = 2475000,
    BreakerHyperAwakeningSkills = 360002,
    BreakerSkills1 = 47000,
    BreakerSkills2 = 360001,
    BreakerSkills3 = 2470500,
    BreakerSkills4 = 2470510,
    BreakerSkills5 = 47001,
    BreakerSkills6 = 360000,
    BreakerIdentitySkills1 = 630104,
    BreakerIdentitySkills2 = 360003,
    DeathbladeSkills = 2251000,
    DeathbladeSkills1 = 410000,
    DeathbladeSkills2 = 2250900,
    DeathbladeAwakeningSkills = 2255000,
    DeathbladeAllSkills1 = 2251101,
    DeathbladeAllSkills2 = 2250200,
    DeathbladeSurgeSkills1 = 1410004,
    DeathbladeSurgeSkills2 = 41000,
    DeathbladeSurgeSkills3 = 41001,
    DeathbladeSurgeSkills4 = 2251100,
    DeathbladeSurgeSkills5 = 2255700,
    DeathbladeZeroSkills = 2250500,
    DeathbladeHyperAwakeningSkills = 2255500,
    DeathbladeAllDeathTranceSurgeSkills = 410004,
    DeathbladeHyperAwakeningTechniqueSkills1 = 2255400,
    DeathbladeHyperAwakeningTechniqueSkills = 2255300,
    DeathbladeAwakeningSkills2 = 410003,
    DeathbladeDeathTranceSurgeSkills1 = 2250400,
    DeathbladeDeathTranceSurgeSkills = 630103,
    AllDeathbladeSkills = 34,
    MachinistAwakeningSkills1 = 2355000,
    MachinistAwakeningSkills2 = 540003,
    MachinistSkills = 54001,
    MachinistSkills1 = 35090,
    MachinistSkills2 = 2350600,
    MachinistSkills3 = 540004,
    MachinistSkills4 = 540001,
    MachinistSkills5 = 540101,
    MachinistSkills6 = 2350800,
    MachinistSkills7 = 54000,
    MachinistSkills8 = 2350500,
    MachinistSkills9 = 540000,
    MachinistSkills10 = 2350701,
    MachinistSkills11 = 2353400,
    MachinistSkills12 = 2350700,
    MachinistSkills13 = 2351101,
    MachinistSkills14 = 540002,
    MachinistCommandSkills = 2351100,
    MachinistAllAwakeningSkills = 2355200,
    MachinistHyperAwakeningSkills = 2355100,
    ArtilleristSkills1 = 14000002,
    ArtilleristSkills2 = 530004,
    ArtilleristSkills3 = 530000,
    ArtilleristSkills4 = 530006,
    ArtilleristSkills5 = 53001,
    ArtilleristSkills6 = 2305000,
    ArtilleristBarrageSkills = 2300400,
    ArtilleristUnnamedSkills = 53000,
    AllArtilleristSkills = 2301000,
    ArtilleristUnnamedSkills1 = 530005,
    ArtilleristAwakeningSkills = 530003,
    ArtilleristAllAwakeningSkills = 2305200,
    ArtilleristHyperAwakeningSkills = 2305100,
    SharpshooterAwakeningSkills = 510003,
    SharpshooterSkills1 = 510000,
    SharpshooterSkills2 = 510005,
    SharpshooterSilverhawkSkills1 = 2284400,
    SharpshooterSilverhawkSkills2 = 2283400,
    SharpshooterSilverhawkSkills3 = 2284100,
    SharpshooterSilverhawkSkills4 = 510004,
    SharpshooterHyperAwakeningTechniqueSkills = 2285000,
    Sharpshooter2Skills = 2285010,
    SharpshooterAllSilverhawkSkills1 = 51000,
    SharpshooterAllSilverhawkSkills = 51001,
    SharpshooterAllAwakeningSkills = 2285200,
    SharpshooterHyperAwakeningSkills = 2285100,
    AllClassSpaceBarSkills = 991100,
    AllClassSpaceBarSkills1 = 991101,
    AllClassStandUpSkills1 = 1410005,
    AllClassStandUpSkills2 = 991200,
    SupportIdentityAndHyperAwakeningTechniqueSkills = 16000001,
}

pub static SKILL_GROUPS: Lazy<FxHashSet<u32>> = Lazy::new(|| {
    // SkillGroup::iter().collect()
    SkillGroup::iter().map(|sg| sg as u32).collect()
});

impl<'de> Deserialize<'de> for SkillGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;

        if SKILL_GROUPS.contains(&value) {
            return unsafe {
                Ok(std::mem::transmute(value))
            }    
        }

        Ok(SkillGroup::None)
    }
}

impl<'de> Deserialize<'de> for SkillGroup1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;

        Ok(match value {

            _ => SkillGroup1::None
        })
    }
}
