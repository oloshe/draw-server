use actix::MailboxError;
use actix_web_actors::ws::WebsocketContext;

use crate::ws::conn::WsConn;
use utils::error;


pub(crate) trait WebSocketExtend {
	fn succ(&mut self, model: &str);
}

impl WebSocketExtend for WebsocketContext<WsConn> {
    fn succ(&mut self, model: &str) {
        self.text(format!("{}.succ", model));
    }
}

pub(crate) trait SimplifyMailbox<T> {
	fn handle<F>(self, handler: F)
	where
		F: FnOnce(T) -> ();
}

impl<T> SimplifyMailbox<T> for Result<T, MailboxError> {
    fn handle<F>(self, handler: F)
	where
		F: FnOnce(T) -> () 
	{
		match self {
			Ok(data) => {
				handler(data);
			},
			Err(e) => {
				error!("{}", e);
			}
		}
    }
}
