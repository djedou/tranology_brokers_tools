use eq_float::F64;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Zone {
    pub id: i64,
    pub mitigated: bool,
    pub start: f64,
    pub end: f64,
    pub zone_type: ZoneType
}

impl Eq for Zone {}

impl Hash for Zone {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        F64(self.start).hash(state);
        F64(self.end).hash(state);
        self.zone_type.hash(state);
    }
}

#[derive(Debug, Clone, Default, PartialEq, Hash)]
pub enum ZoneType {
    #[default]
    None,
    BearishRejBlock,
    BullishRejBlock,
    Support,
    Resistance
}

impl Eq for ZoneType {}

impl Zone {
    pub fn new(id: i64, start: f64, end: f64, zone_type: ZoneType) -> Zone {
        Zone {
            id,
            mitigated: false,
            start,
            end,
            zone_type
        }
    }
}