use actix::{Actor, Addr};

use self::actor::LobbyActor;

pub mod actor;

lazy_static::lazy_static! {
    pub static ref LOBBY_ADDR: Addr<LobbyActor> = LobbyActor::start_default();
}