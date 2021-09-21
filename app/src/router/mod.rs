use actix_web_actors::ws::WebsocketContext;
use utils::error;

use crate::ws::{
    request::WsRequest,
    conn::WsConn,
};

#[macro_use]
pub mod constraints;
pub mod model;

// 为了给 ctx address方法，需要引入
use actix::AsyncContext;
pub(crate) fn run_cmd(req: WsRequest, ctx: &mut WebsocketContext<WsConn>) {
    let WsRequest { model, data } = req;
    if let Some(call_function) = crate::router::model::MODEL_FUNC.get(model.as_str()) {
        let addr = ctx.address();
        call_function(addr, data);
    } else {
        error!("Model Error: no model call [{}]", model);
        todo!("发送失败响应")
    }
}