
/// 玩家通过ws发送过来的请求，分割为两部分
/// 第一部分确认函数，第二部分为携带的数据
pub struct WsRequest {
	pub model: String,
	pub data: String,
}
impl WsRequest {
    fn new(model: &str, data: &str) -> Self {
        WsRequest { model: model.to_string(), data: data.to_string() }
    }
    /// 转换 socket 接收到的文本数据，格式为
    pub(crate) fn from_str(text: &str) -> Self {
        info!("parse: {}", text);
        let text = text.trim();
        let pos = text.find(".");
        if let Some(pos) = pos {
            let (model, data) = text.split_at(pos);
            Self::new(model, data)
        } else {
            Self::new(text, "")
        }
    }
}

use std::borrow::Cow;

use utils::info;

use super::conn::WsConn;


/// 连接时的 query 参数
#[derive(Debug)]
pub struct ConnectQuery {
    /// 微信小程序登陆的 code
    pub code: Option<String>,
    /// web 端登录时的 uid
    pub uid: Option<String>,
    /// web 端登陆时的密码
    pub pwd: Option<String>,
}
impl ConnectQuery {
    pub fn new(query_string: &str) -> Self {
        let queries = utils::querystring::querify(query_string);
        let mut result = ConnectQuery { 
            uid: None,
            code: None,
            pwd: None,
        };
        queries.iter().for_each(|pair| {
            let (key, value) = *pair;
            if value == "" {
                return;
            }
            let some_value = Some(
                utils::urlencoding::decode(&value)
                    .unwrap_or(Cow::Borrowed(value))
                    .to_string()
            );
            match key {
                "uid" => result.uid = some_value,
                "code" => result.code = some_value,
                "pwd" => result.pwd = some_value,
                _ => (),
            };
        });
        result
    }
}

pub struct QueryReducer;

impl QueryReducer{
	pub(crate) fn exec(query: ConnectQuery) -> WsConn {
		if let Some(uid) = &query.uid {
			
		}
		WsConn::default()
	}
}