use actix_web::{HttpRequest, HttpResponse, web};
use utils::info;

use crate::{WsConn, ws::request::ConnectQuery};

/// ws 连接时调用
pub async fn ws_upgrade(
    req: HttpRequest, 
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    // query 参数转换
    let connect_query = ConnectQuery::new(req.query_string());
    info!("connect {:?}", connect_query);
    let conn = WsConn::default();
    // 开始 websocket 连接进行升级
	let resp = actix_web_actors::ws::start(conn , &req, stream);
    resp
}