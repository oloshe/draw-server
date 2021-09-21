use actix::{Actor, Addr};

use actor::LobbyActor;

pub(crate) mod actor;

lazy_static::lazy_static! {
    pub static ref LOBBY_ADDR: Addr<LobbyActor> = LobbyActor::start_default();
}