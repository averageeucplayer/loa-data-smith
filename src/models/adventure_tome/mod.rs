pub mod rethramis;
pub mod yudia;
pub mod east_luterra;
pub mod west_luterra;
pub mod tortoyk;
pub mod anikka;
pub mod arthetine;
pub mod shushire;
pub mod north_vern;
pub mod rohendel;
pub mod feiton;
pub mod punika;
pub mod south_vern;
pub mod elgacia;
pub mod voldis;
pub mod south_kurzan;
pub mod north_kurzan;
pub mod rowen;
pub mod yorn;

use crate::models::adventure_tome::rethramis::Rethramis;
use crate::models::adventure_tome::yudia::Yudia;
use crate::models::adventure_tome::anikka::Anikka;
use crate::models::adventure_tome::east_luterra::EastLuterra;
use crate::models::adventure_tome::west_luterra::WestLuterra;
use crate::models::adventure_tome::tortoyk::Tortoyk;
use crate::models::adventure_tome::arthetine::Arthetine;
use crate::models::adventure_tome::north_vern::NorthVern;
use crate::models::adventure_tome::south_vern::SouthVern;
use crate::models::adventure_tome::shushire::Shushire;
use crate::models::adventure_tome::feiton::Feiton;
use crate::models::adventure_tome::punika::Punika;
use crate::models::adventure_tome::south_kurzan::SouthKurzan;
use crate::models::adventure_tome::north_kurzan::NorthKurzan;
use crate::models::adventure_tome::rohendel::Rohendel;
use crate::models::adventure_tome::rowen::Rowen;
use crate::models::adventure_tome::yorn::Yorn;
use crate::models::adventure_tome::elgacia::Elgacia;
use crate::models::adventure_tome::voldis::Voldis;
use crate::{create_struct_with_npcs, Npc};

pub struct Stub {
    pub monster: Box<Stub>,
    pub rapport: Box<Stub>,
    pub bosses: Box<Stub>
}

pub struct AdventureTome<'a> {
    pub rethramis: Rethramis<'a>,
    pub yudia: Yudia<'a>,
    pub west_luterra: WestLuterra<'a>,
    pub east_luterra: EastLuterra<'a>,
    pub tortoyk: Tortoyk<'a>,
    pub anikka: Anikka<'a>,
    pub arthetine: Arthetine<'a>,
    pub north_vern: NorthVern<'a>,
    pub shushire: Shushire<'a>,
    pub rohendel: Rohendel<'a>,
    pub yorn: Yorn<'a>,
    pub feiton: Feiton<'a>,
    pub punika: Punika<'a>,
    pub south_vern: SouthVern<'a>,
    pub rowen: Rowen<'a>,
    pub elgacia: Elgacia<'a>,
    pub voldis: Voldis<'a>,
    pub south_kurzan: SouthKurzan<'a>,
    pub north_kurzan: NorthKurzan<'a>,
}