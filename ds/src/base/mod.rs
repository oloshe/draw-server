use std::{fmt::Display, hash::Hash};
use serde::Serialize;


#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub struct Uid(pub i64);

// 继承 i64的格式化
impl Display for Uid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        i64::fmt(&self.0, f)
    }
}

impl Hash for Uid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}


#[derive(Debug, PartialEq, Eq)]
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




#[derive(Debug)]
pub struct SeatNo(pub u16);