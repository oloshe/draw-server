use actix_web::{HttpResponse, post, web};
use serde::Deserialize;

use crate::data_struct::base::RoomId;
use crate::data_struct::{player::PlayerData, room::RoomInfo};
use crate::msg::message::{LobbyCreateRoom, LobbyJoinRoom};
use crate::LOBBY_ADDR as lobby;

#[derive(Debug,Deserialize)]
pub struct ImportedData {
	pub player: PlayerData,
}

#[post("/join/{room_id}")]
pub(crate) async fn join_room(room_id: web::Path<String>, data: web::Json<ImportedData>) -> HttpResponse {
	let ret = lobby.send(LobbyJoinRoom {
		player_data: data.0.player,
		room_id: RoomId(room_id.to_string()),
	}).await;
	match ret {
		Ok(ret) => {
			if let Some(reason) = ret {
				HttpResponse::Ok()
					.body(reason)
			} else {
				HttpResponse::Ok()
					.body("ok")
			}
		},
		Err(e) => {
			utils::error!("{}", e);
			HttpResponse::InternalServerError()
				.body("fail")
		}
	}
}

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