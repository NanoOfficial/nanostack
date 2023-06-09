/**
 * @filename: entity_id.rs
 * @author: Krisna Pranav
 * @copyright: COPYRIGHT (2023) MIT LICENSE Krisna Pranav
*/
use crate::*;
use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntityId(Ulid);

impl EntityId {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self(Ulid::generate())
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for EntityId {
    type Err = DecodingError;

    fn from_str(entity_id: &str) -> Result<Self, Self::Err> {
        Ok(EntityId(entity_id.parse()?))
    }
}

#[cfg(feature = "serde-lite")]
impl Serialize for EntityId {
    fn serialize(&self) -> Result<serde_lite::Intermediate, serde_lite::Error> {
        self.0.to_string().serialize()
    }
}

#[cfg(feature = "serde-lite")]
impl Deserialize for EntityId {
    fn deserialize(intermediate: &serde_lite::Intermediate) -> Result<Self, serde_lite::Error> {
        intermediate
            .as_str()
            .ok_or_else(|| {
                serde_lite::Error::invalid_value("EntityId can be deserialized only from String")
            })?
            .parse()
            .map_err(|error| serde_lite::Error::invalid_value(error))
            .map(Self)
    }
}
