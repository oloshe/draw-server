use actix::MailboxError;
use utils::error;

mod room_model;

fn handle_msg_request<T, F>(ret: Result<T, MailboxError>, handle_succ: F)
where
	F: FnOnce(T) -> ()
{
	match ret {
		Ok(data) => {
			handle_succ(data);
		},
		Err(e) => {
			error!("{}", e);
		}
	}
}
