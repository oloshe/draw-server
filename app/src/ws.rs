use actix::{Actor, AsyncContext, Handler, StreamHandler};
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws::{self, WebsocketContext};
use ds::base::{RoomId, Uid};
use ds::conn::ConnectQuery;
use utils::info;

use crate::lobby::LOBBY_ADDR;

use crate::msg::message::*;

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
    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        LOBBY_ADDR.do_send(LobbyDisconnect {
            uid: self.uid,
        });
        actix::Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
				let req = parse_text(&text.to_string());
				crate::router::run_cmd(req, ctx);
            },
			Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
			Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

/// ws 连接时调用
pub async fn ws_index(
    req: HttpRequest, 
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    // query 参数转换
    let connect_query = ConnectQuery::new(req.query_string());
    info!("connect {:?}", connect_query);
    let conn = WsConn::default();
    // 开始 websocket 连接进行升级
	let resp = ws::start(conn , &req, stream);
    resp
}




pub struct WsRequest {
	pub model: String,
	pub data: String,
}
/// 转换 socket 接收到的文本数据，格式为
/// \<model>.\<func>.\<data(json)>
fn parse_text(text: &String) -> WsRequest {
	info!("parse: {}", text);
	let text = text.trim();
	let vec: Vec<&str> = text.splitn(2, ".").collect();
	let get_param = |index: usize, default: &str| vec.get(index).unwrap_or(&default).to_string();
	let model = get_param(0, "");
	let data = get_param(1, "");
	WsRequest { model, data }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0)
    }
}