use actix::AsyncContext;
use actix::{Actor, Handler, StreamHandler};
use actix_web_actors::ws::{self, WebsocketContext, Message};

use crate::base::*;
use crate::LOBBY_ADDR;
use crate::msg::message::*;
use crate::ws::request::WsRequest;

pub struct WsConn {
    pub room_id: Option<RoomId>,
    pub uid: Uid,
}

impl Default for WsConn {
    fn default() -> Self {
        Self { room_id: Default::default(), uid: Uid(0) }
    }
}

impl Actor for WsConn {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        LOBBY_ADDR.do_send(LobbyConnect {
            addr: ctx.address().recipient(),
            uid: self.uid,
        })
    }
    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        LOBBY_ADDR.do_send(LobbyDisconnect {
            uid: self.uid,
        });
        actix::Running::Stop
    }
}

impl StreamHandler<Result<Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Text(text)) => {
				let req = WsRequest::from_str(&text); // 自动解引用
				crate::router::run_cmd(req, ctx);
            },
			Ok(Message::Ping(msg)) => ctx.pong(&msg),
            Ok(Message::Binary(bin)) => ctx.binary(bin),
			Ok(Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0)
    }
}