use serde::{Serialize, Deserialize};
use utils::serde_repr::*;
#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum GameType {
    /// 经典你画我猜
    YouDrawIGuess = 1,
    /// 画猜接龙
    DrawGuessChain = 2,
}