use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, Npc};


pub struct Rethramis<'a> {
    pub neria: &'a Npc,
    pub siera: &'a Npc,
    pub rudric: &'a Npc
}

pub struct Stub {
    pub monster: Box<Stub>,
    pub rapport: Box<Stub>,
    pub bosses: Box<Stub>
}

pub struct AdventureTome {
    pub rethramis: Stub,
    pub yudia: Stub,
    pub west_luterra: Stub,
    pub east_luterra: Stub,
    pub tortoyk: Stub,
    pub anikka: Stub,
    pub arthetine: Stub,
    pub north_vern: Stub,
    pub shushire: Stub,
    pub rohendel: Stub,
    pub yorn: Stub,
    pub feiton: Stub,
    pub punika: Stub,
    pub south_vern: Stub,
    pub rowen: Stub,
    pub elgacia: Stub,
    pub voldis: Stub,
    pub south_kurzan: Stub,
    pub north_kurzan: Stub,
}