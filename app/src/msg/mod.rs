pub mod message;
pub mod macros;

pub type NormalResult = Option<&'static str>;

#[cfg(test)]
mod test {
    use super::message::*;
    #[test]
    fn test_msg_1() {
        let msg = WsMessage("test".to_string());
        assert_eq!(msg.0, "test".to_string())
    }
}