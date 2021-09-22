use actix::Addr;
use actix_redis::RedisActor;

utils::lazy_static!{
	pub static ref REDIS: Addr<RedisActor> = {
		let redis_url = "127.0.0.1:6379";
		let redis_addr = RedisActor::start(redis_url);
		redis_addr
	};
}