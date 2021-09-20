use actix::AsyncContext;
use actix_web_actors::ws::WebsocketContext;
use utils::error;

use crate::ws::{WsConn, WsRequest};

pub mod constraints;
pub mod model;

pub fn run_cmd(req: WsRequest, ctx: &mut WebsocketContext<WsConn>) {
    let WsRequest { model, data } = req;
    if let Some(call_function) = crate::router::model::MODEL_FUNC.get(model.as_str()) {
        call_function(ctx.address(), data);
    } else {
        error!("Model Error: no model call [{}]", model);
        todo!("发送失败响应")
    }
}