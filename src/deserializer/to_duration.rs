use chrono::{Duration, TimeDelta};
use serde::{Deserialize, Deserializer};

pub fn to_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let value = i64::deserialize(deserializer)?;
    if value == -1 {
        Ok(TimeDelta::MAX)
    } else if value == 0 {
        Ok(TimeDelta::MAX)
    } else {
        Ok(Duration::seconds(value))
    }
}