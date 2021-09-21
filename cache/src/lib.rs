use redis::{Client, Commands};

pub mod user;

utils::lazy_static! {
	pub(crate) static ref REDIS: Client = create_redis_client();
}

fn create_redis_client() -> Client {
	let host = "127.0.0.1";
	let port = 6379;
	let addr = format!("redis://{}:{}",host,port);
	let client = Client::open(addr).unwrap();
	client
}

/// 获取金币
pub fn get_gold(uid: &i64) -> i64 {
	let mut conn = REDIS.get_connection().unwrap();
	let gold = conn.get(format!("{}:gold", uid)).unwrap_or(0i64);
	gold
}

/// 改变金币
pub fn change_gold(uid: &i64, delta: i64) -> Result<i64,()> {
	let mut conn = REDIS.get_connection().unwrap();
	let gold: i64 = conn.incr(format!("{}:gold", uid), delta).unwrap();
	Ok(gold)
}