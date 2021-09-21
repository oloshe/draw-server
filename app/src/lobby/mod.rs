use actix::{Actor, Addr};

use actor::LobbyActor;

pub(crate) mod actor;

utils::lazy_static! {
    pub static ref LOBBY_ADDR: Addr<LobbyActor> = LobbyActor::start_default();
}