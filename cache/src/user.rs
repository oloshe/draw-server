use utils::log::error;
use crate::REDIS;
use redis::Commands;

static ID_KEY: &'static str = "current:id";
static INIT_ID: i64 = 1000;

pub fn get_id() -> i64 {
	let mut conn = REDIS.get_connection().unwrap();
	let id = conn.get(ID_KEY).unwrap_or_else(|e| {
		error!("redis get id error: {}", e);
		let _: () = conn.set(ID_KEY, INIT_ID).unwrap();
		INIT_ID
	});
	let _: () = conn.set(ID_KEY, id + 1).unwrap();
	id
}