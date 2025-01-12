use chapter_one_earth_shattering_hellfire::ChapterOneEarthShatteringHellfire;
use overture_spiral_of_the_crimson_midnight_sun::OvertureSpiralOfTheCrimsonMidnightSun;

use crate::wrap_npc_struct;

pub mod chapter_one_earth_shattering_hellfire;
pub mod overture_spiral_of_the_crimson_midnight_sun;

wrap_npc_struct!(KazerosRaid,
{
    overture: OvertureSpiralOfTheCrimsonMidnightSun<'a>,
    chapter_one: ChapterOneEarthShatteringHellfire<'a>
});