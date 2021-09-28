#[macro_use]
mod msg;
mod ws;
mod lobby;
mod user;
mod router;
mod start;
mod http;

mod data_struct;

pub use start::create_app;

pub(crate) use data_struct::base;
pub(crate) use ws::conn::WsConn;
pub(crate) use lobby::LOBBY_ADDR;
