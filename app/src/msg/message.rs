use actix::{Message, Recipient};
use crate::base::*;
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
    } -> (),

    /// 大厅接收玩家失去连接消息
    LobbyDisconnect {
        uid: Uid,
    } -> (),
    
    /// 大厅接受玩家加入房间消息
    LobbyJoinRoom {
        uid: Uid,
        room_id: RoomId,
    } -> NormalResult,
}
