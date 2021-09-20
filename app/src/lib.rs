pub mod msg;
pub mod ws;
pub mod lobby;
pub mod user;
pub mod router;

use actix_web::{App, HttpServer, web};

use crate::ws::ws_index;

pub async fn create_app() {
    let (addr, port) = ("0.0.0.0", "8000");

    let server_result = HttpServer::new(|| {
        App::new()
            .service(web::resource("/ws").route(web::get().to(ws_index)))
    })
        .bind(format!("{}:{}", addr, port))
        .expect(format!("Can't bind to port {}", port).as_str())
        .run()
        .await;
    
    if let Err(e) = server_result {
        utils::log::error!("Error: {:?}", e)
    }
}