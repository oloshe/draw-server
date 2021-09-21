use std::{collections::HashMap};

use actix::{Actor, Context, Handler, Recipient};
use ds::{base::{RoomId, Uid}, room::RoomInfo};
use utils::{error, info};

use crate::msg::{NormalResult, message::*};

type Socket = Recipient<WsMessage>;
pub struct LobbyActor {
    /// 根据 uid 找到 地址
    pub sessions: HashMap<Uid, Socket>,
    /// 在线数量
    pub online_count: u32,
    /// 房间 id-信息 映射表
    pub rooms: HashMap<RoomId, RoomInfo>,
    // 玩家在哪个房间
    pub player_room: HashMap<Uid, RoomId>, 
}

impl Actor for LobbyActor {
    type Context = Context<Self>;
}

impl Default for LobbyActor {
    fn default() -> Self {
        Self { 
            online_count: 0,
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            player_room: HashMap::new(),
        }
    }
}

impl LobbyActor {
    fn send_msg(&self, msg: String, uid: &Uid) {
        if let Some(socket) = self.sessions.get(uid) {
            let _ = socket.do_send(WsMessage(msg));
        } else {
            info!("attempting to send message but couldn't find user uid: {}.", uid);
        }
    }

    fn send_msg_to_room(&self, msg: String, room_id: &RoomId, expect_uid: Option<Uid>) {
        let room_info = self.rooms.get(room_id);
        if let Some(room_info) = room_info {
                room_info.players.iter()
                    .filter(|(_, p)| Some(p.uid) == expect_uid)
                    .for_each(|data| self.send_msg(msg.to_owned(), data.0))
        } else {
            error!("send message to room error");
        }
    }

    fn remove_player(&mut self, uid: Uid) {
        // 删除玩家房间
        if let Some(room_id) = self.player_room.remove(&uid) {
            if let Some(_) = self.rooms.get_mut(&room_id) {
                
            } else {
                self.rooms.remove(&room_id); // 空房间则移除该房间
            }
        }
    }
}

impl Handler<LobbyConnect> for LobbyActor {
    type Result = ();

    fn handle(&mut self, msg: LobbyConnect, _: &mut Self::Context) -> Self::Result {
        info!("{} connected!", &msg.uid);
        self.sessions.insert(msg.uid, msg.addr);
        self.online_count += 1;
    }
}

impl Handler<LobbyDisconnect> for LobbyActor {
    type Result = ();

    fn handle(&mut self, msg: LobbyDisconnect, _: &mut Self::Context) -> Self::Result {
        info!("{} disconnect.", msg.uid);
        self.online_count = self.online_count - 1;
        // 移除 sessions
        if self.sessions.remove(&msg.uid).is_some() {
            self.remove_player(msg.uid);
        }
    }
}

impl Handler<LobbyJoinRoom> for LobbyActor {
    type Result = NormalResult;

    fn handle(&mut self, msg: LobbyJoinRoom, ctx: &mut Self::Context) -> Self::Result {
        
        None
    }
}