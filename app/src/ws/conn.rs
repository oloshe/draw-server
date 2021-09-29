use actix::{ActorContext, ContextFutureSpawner, fut};
use actix::{ActorFutureExt, AsyncContext, WrapFuture};
use actix::{Actor, Handler, StreamHandler};
use actix_web_actors::ws::{self, CloseReason, Message, WebsocketContext};

use crate::base::*;
use crate::LOBBY_ADDR;
use crate::msg::message::*;
use crate::ws::request::WsRequest;

pub struct WsConn {
    pub room_id: RoomId,
    pub uid: Uid,
}

impl Actor for WsConn {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        LOBBY_ADDR.send(LobbyConnect {
            addr: ctx.address().recipient(),
            uid: self.uid.clone(),
			room_id: self.room_id.clone(),
        })
			.into_actor(self)
			.then(|ret, conn, ctx| {
				match ret {
					Ok(false) => {
						ctx.text("invalid");
						ctx.close(Some(
							CloseReason {
								code: ws::CloseCode::Invalid,
								description: None
							}
						));
					},
					_ => (),
				}
				fut::ready(())
			})
			.wait(ctx);
    }
    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        LOBBY_ADDR.do_send(LobbyDisconnect {
            uid: self.uid.clone(),
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