use actix_web::{HttpRequest, HttpResponse, web, get};
use utils::info;
use serde::Deserialize;

use crate::data_struct::base::{RoomId, Uid};
use crate::data_struct::player::PlayerData;
use crate::LOBBY_ADDR as lobby;
use crate::msg::message::LobbyJoinRoom;

use super::conn::WsConn;

// use crate::{ws::request::{ConnectQuery, QueryReducer}};

// /// ws 连接时调用
// pub async fn ws_upgrade(
//     req: HttpRequest, 
//     stream: web::Payload,
// ) -> Result<HttpResponse, actix_web::Error> {
//     // query 参数转换
//     // let connect_query = ConnectQuery::new(req.query_string());
//     // info!("connect {:?}", connect_query);
//     // let conn = QueryReducer::exec(connect_query);
//     // 开始 websocket 连接进行升级
// 	let resp = actix_web_actors::ws::start(conn , &req, stream);
//     resp
// }


#[derive(Debug, Deserialize)]
pub struct ImportedData {
	pub player: PlayerData,
}

#[get("/connect/{room_id}/{uid}")]
pub(crate) async fn join_room(
	params: web::Path<(String, String)>,
	req: HttpRequest,
	stream: web::Payload,
) -> HttpResponse {
	let room_id = RoomId(params.0.to_string());
	let uid = Uid(params.1.to_string());
	info!("{} / {}", &room_id, &uid);
	let conn = WsConn { room_id, uid };

	let resp = actix_web_actors::ws::start(conn , &req, stream);

	match resp {
		Ok(ret) => ret,
		Err(e) => e.error_response(),
	}
	
}