use actix::ActorFutureExt;
use actix::ContextFutureSpawner;
use actix::Handler;
use actix::WrapFuture;
use actix::fut;

use utils::{info, error};
use crate::WsConn;
use crate::LOBBY_ADDR;
use crate::router::model::*;
use crate::msg::message::*;
use crate::extend::*;
use super::handle_msg_request;

impl Handler<RoomReady> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: RoomReady, ctx: &mut Self::Context) -> Self::Result {
		let ready = msg.raw == "true";
        LOBBY_ADDR.send(LobbyRoomReady {
			room_id: self.room_id.clone(),
			uid: self.uid.clone(),
			ready,
		})
			.into_actor(self)
			.then(move |ret, _, ctx|{
				ret.handle(|_| {
					ctx.succ(&msg.model);
				});
				fut::ready(())
			})
			.wait(ctx);
    }
}
