use actix::{Message, Recipient};
use crate::{base::*, data_struct::{player::PlayerData, room::RoomInfo}};
use super::NormalResult;

msg! {
    /// 通用发送消息 msg
    WsMessage(String) -> (),
}

msg! {
    /// 大厅接收连接消息
    LobbyConnect {
        /// 玩家的ws接收器
        addr: Recipient<WsMessage>,
        uid: Uid,
		room_id: RoomId,
    } -> bool,

    /// 大厅接收玩家失去连接消息
    LobbyDisconnect {
        uid: Uid,
    } -> (),

	/// 创建房间
	LobbyCreateRoom {
		room_info: RoomInfo,
	} -> (),
    
    /// 加入房间
    LobbyJoinRoom {
        player_data: PlayerData,
        room_id: RoomId,
    } -> NormalResult,

	LobbyRoomReady {
		room_id: RoomId,
		uid: Uid,
		ready: bool,
	} -> ()
}
