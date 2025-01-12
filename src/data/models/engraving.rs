use crate::deserializer::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Clone)]
pub struct RawEngraving<'a> {
    pub id: u32,
    pub name: Option<&'a str>,
    pub icon: Option<&'a str>,
}
