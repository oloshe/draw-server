use std::{fmt::Display, hash::Hash};
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Uid(pub String);

impl Default for Uid {
    fn default() -> Self {
        Self(Default::default())
    }
}

// 继承 i64的格式化
impl Display for Uid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        String::fmt(&self.0, f)
    }
}

impl Hash for Uid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RoomId(pub String);

impl Display for RoomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Hash for RoomId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}




#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy)]
pub struct SeatNo(pub u16);

impl From<u16> for SeatNo {
    fn from(num: u16) -> Self {
        Self(num)
    }
}