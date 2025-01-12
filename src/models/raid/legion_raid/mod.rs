pub mod valtan;
pub mod vykas;
pub mod kakul_saydon;
pub mod brelshaza;
pub mod akkan;
pub mod thaemine;

use valtan::Valtan;
use vykas::Vykas;
use kakul_saydon::KakulSaydon;
use brelshaza::Brelshaza;
use akkan::Akkan;
use thaemine::Thaemine;
use crate::wrap_npc_struct;

wrap_npc_struct!(LegionRaid,
{
    valtan: Valtan<'a>,
    vykas: Vykas<'a>,
    kakul_saydon: KakulSaydon<'a>,
    brelshaza: Brelshaza<'a>,
    akkan: Akkan<'a>,
    thaemine: Thaemine<'a>
});