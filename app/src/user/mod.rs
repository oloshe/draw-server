use actix::Handler;

use crate::ws::WsConn;

use crate::router::model::*;

impl Handler<UserNick> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: UserNick, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

impl Handler<UserAvatar> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: UserAvatar, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}