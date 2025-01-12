use chaos_guardian_purification::ChaosGuardianPurification;
use rustc_hash::FxHashMap;
use vaigrys_test::VaigrysTest;

use crate::wrap_npc_struct;

pub mod vaigrys_test;
pub mod chaos_guardian_purification;

wrap_npc_struct!(GuardianRaid,
{
    vaigrys_test: VaigrysTest<'a>,
    chaos_guardian_purification: ChaosGuardianPurification<'a>
});
