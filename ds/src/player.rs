use crate::base::Uid;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PlayerData {
    pub uid: Uid,
    pub nick: String,
    pub avatar: String,
}