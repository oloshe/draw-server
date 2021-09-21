use actix_web::{App, HttpServer, web};
use crate::ws::upgrade::ws_upgrade;

/// 启动服务器
pub async fn create_app() {
    let (addr, port) = ("0.0.0.0", "8000");

    let server_result = HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/ws").route(
                    web::get().to(ws_upgrade)
                )
            )
    })
        .bind(format!("{}:{}", addr, port))
        .expect(format!("Can't bind to port {}", port).as_str())
        .run()
        .await;
    
    if let Err(e) = server_result {
        utils::log::error!("Error: {:?}", e)
    }
}