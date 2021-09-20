use std::borrow::Cow;


/// 连接时的 query 参数
#[derive(Debug)]
pub struct ConnectQuery {
    /// 微信小程序登陆的 code
    pub code: Option<String>,
    /// web 端登录时的 nick
    pub nick: Option<String>,
    /// web 端登陆时的密码
    pub pwd: Option<String>,
}
impl ConnectQuery {
    pub fn new(query_string: &str) -> Self {
        let queries = querystring::querify(query_string);
        let mut result = ConnectQuery { 
            nick: None,
            code: None,
            pwd: None,
        };
        queries.iter().for_each(|pair| {
            let (key, value) = *pair;
            if value == "" {
                return;
            }
            let some_value = Some(
                urlencoding::decode(&value)
                    .unwrap_or(Cow::Borrowed(value))
                    .to_string()
            );
            match key {
                "un" => result.nick = some_value,
                "code" => result.code = some_value,
                "pwd" => result.pwd = some_value,
                _ => (),
            };
        });
        result
    }
}