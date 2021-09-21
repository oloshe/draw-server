use crate::base::Uid;
use serde::Serialize;

/// 玩家基础数据
#[derive(Debug, Clone, Serialize)]
pub struct PlayerData {
    /// 唯一id
    pub uid: Uid,
    /// 昵称
    pub nick: String,
    /// 头像
    pub avatar: String,
}

/// 玩家从网页登陆信息
pub struct PlayerWebLoginData {
    pub uid: Uid,
    /// 应该会有密码
    pub pwd: Option<String>,
}

/// 玩家从微信登陆的信息
pub struct PlayerWechatLoginData {
    /// 通过 wx.login 拿到的 code，再去微信服务器拿到 openid
    /// 再去数据库查找到玩家数据
    pub code: String
}