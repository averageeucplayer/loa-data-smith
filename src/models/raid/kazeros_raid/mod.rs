pub mod chapter_one_earth_shattering_hellfire;
pub mod overture_spiral_of_the_crimson_midnight_sun;
pub mod act_two_requiem_of_the_floating_nightmare;

use chapter_one_earth_shattering_hellfire::ChapterOneEarthShatteringHellfire;
use overture_spiral_of_the_crimson_midnight_sun::OvertureSpiralOfTheCrimsonMidnightSun;
use act_two_requiem_of_the_floating_nightmare::ActTwoRequiemOfTheFloatingNightmare;

use crate::wrap_npc_struct;

wrap_npc_struct!(KazerosRaid,
{
    overture: OvertureSpiralOfTheCrimsonMidnightSun<'a>,
    chapter_one: ChapterOneEarthShatteringHellfire<'a>,
    act_two: ActTwoRequiemOfTheFloatingNightmare<'a>
});