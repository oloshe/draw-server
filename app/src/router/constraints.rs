use actix::Addr;
use std::collections::HashMap;
use crate::WsConn;

/// 使用 `#[derive(Message, Cmd)]` 实现该模块 并且再 [`MODEL_FUNC`] 定义处注册路由
pub trait ModelFunc {
	fn func() -> &'static str;
	fn route(addr: Addr<WsConn>, data: String);
	/// 注册路由默认实现，请勿重写。
	fn register(map: &mut HashMap<&'static str, fn(addr: Addr<WsConn>, data: String)>) {
		map.entry(Self::func())
			.or_insert(Self::route);
	}
}

///  注册路由
macro_rules! register_macro {
	($(
		$(#[$outer:meta])*
        $model:ident$(,)?)*
    ) => {
        use std::collections::HashMap;
        use actix::{Addr, Message};
        use crate::router::constraints::ModelFunc;
        use crate::WsConn;

        $(
            #[derive(Message)]
            #[rtype(result = "()")]
            $(#[$outer])*
            pub struct $model {
                // 来自于哪个模块
                pub model: &'static str,
                // 接受自玩家的原始字符串
                pub raw: String,
            }
            impl ModelFunc for $model {
                fn func() -> &'static str { stringify!($model) }
                fn route(addr: Addr<WsConn>, data: String) {
                    addr.do_send(Self {
                        model: stringify!($model),
                        raw: data,
                    })
                }
            }
        )*
        
        lazy_static::lazy_static! {
            pub static ref MODEL_FUNC: HashMap<&'static str, fn(addr: Addr<WsConn>, data: String)> = {
                let mut map = HashMap::new();
                $($model::register(&mut map);)*
                map
            };
        }
    }
}