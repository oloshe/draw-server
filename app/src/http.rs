use actix_web::{HttpResponse, post, web};
use serde::Deserialize;

use crate::data_struct::{player::PlayerData, room::RoomInfo};
use crate::msg::message::{LobbyCreateRoom};
use crate::LOBBY_ADDR as lobby;


#[derive(Debug,Deserialize)]
pub struct CreateRoomInfo {
	pub player: PlayerData,
}

/// 创建房间
#[post("/create")]
pub(crate) async fn create_room(data: web::Json<CreateRoomInfo>) -> HttpResponse {
	let room_info = RoomInfo::new(data.0.player);
	let ret = lobby.send(LobbyCreateRoom { room_info: room_info.clone() }).await;
	match ret {
		Ok(_) => {
			HttpResponse::Ok()
				.json(room_info.clone())
		},
		Err(e) => {
			utils::error!("{}", e);
			HttpResponse::InternalServerError()
				.body("fail")
		}
	}
}