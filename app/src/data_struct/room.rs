use std::collections::HashMap;
use crate::base::*;
use super::{game::GameType, player::PlayerData};

/// 房间信息
pub struct RoomInfo {
    /// 房间 id
    pub room_id: RoomId,
    /// 房间名
    pub room_name: String,
    /// 房主id， None表示没有房主
    pub owner_id: Option<Uid>,
    /// 座位
    pub seat: HashMap<SeatNo, Uid>,
    /// 玩家信息映射表
    pub players: HashMap<Uid, PlayerData>,
    /// 玩家准备状态
    pub ready_state: HashMap<Uid, bool>,
    /// 房间最大人数
    pub max: u16,
    /// 是否允许观战
    pub allow_ob: bool,
    /// 允许观战最大人数 考虑到性能，这里需要限制
    pub max_ob_num: u16,
    /// 房间状态
    pub state: RoomState,
    /// 游戏模式
    pub game: GameType,
}

/// 房间状态
pub enum RoomState {
    Waitting = 1,
    Started = 2,
}