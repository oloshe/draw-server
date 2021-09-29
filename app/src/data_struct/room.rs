use std::{collections::HashMap};
use crate::base::*;
use super::{game::GameType, player::PlayerData};
use serde::{Serialize, Deserialize};

/// 房间信息
#[derive(Debug, Serialize, Clone)]
pub struct RoomInfo {
    /// 房间 id
    pub room_id: RoomId,
    /// 房间名
    pub room_name: String,
	/// 房间描述
	pub desc: String,
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

impl RoomInfo {
	pub(crate) fn new(player: PlayerData,) -> Self {
		Self {
			room_id: RoomId(uuid::Uuid::new_v4().to_string()),
			room_name: format!("{}的房间", &player.nick.clone()),
			owner_id: Some(player.uid.clone()),
			desc: "".to_string(),
			seat: HashMap::new(),
			ready_state: HashMap::new(),
			players: HashMap::new(),
			allow_ob: true,
			max: 10,
			max_ob_num: 10,
			state: RoomState::Waitting,
			game: GameType::YouDrawIGuess,
		}
	}

	pub(crate) fn join(&mut self, player: &PlayerData) -> Option<&'static str> {
		if let Some(seat_no) = self.get_empty_seat() {
			let uid = player.uid.clone();
			self.seat.insert(seat_no, uid.clone());
			self.ready_state.insert(uid.clone(), false);
			self.players.insert(uid.clone(), player.clone());
			None
		} else {
			Some("full")
		}
	}
	fn get_empty_seat(&self) -> Option<SeatNo> {
		for no in 0..self.max {
			let other_id = self.seat.get(&SeatNo(no));
			if let None = other_id {
				return Some(SeatNo(no))
			}
		}
		None
	}
	pub(crate) fn is_this_player_in(&self, uid: &Uid) -> bool {
		self.players.contains_key(uid)
	}
	pub(crate) fn player_ready_change(&mut self, uid: &Uid, ready: bool) {
		if let Some(ready_state) = self.ready_state.get_mut(uid) {
			if &ready == ready_state {
				*ready_state = ready;
			}
		}
	}
}

use utils::serde_repr::*;
/// 房间状态
#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum RoomState {
    Waitting = 1,
    Started = 2,
}