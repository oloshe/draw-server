use actix_web::{App, HttpServer, middleware, web};
use crate::{http::{create_room}, ws::upgrade::{self, join_room}};

/// 启动服务器
pub async fn create_app() {
    let (addr, port) = ("0.0.0.0", "8000");
	let workers = 2;
    let server_result = HttpServer::new(move || {
        App::new()
			.wrap(
				middleware::DefaultHeaders::new()
				.header("server-version", "0.1")
				.header("power-by", "actix-web")
			)
            // .service(
            //     web::resource("/ws").route(
            //         web::get().to(ws_upgrade)
            //     )
            // )
			.service(join_room)
			.service(create_room)
    })
		.workers(workers)
        .bind(format!("{}:{}", addr, port))
        .expect(format!("Can't bind to port {}", port).as_str())
        .run()
        .await;
    
    if let Err(e) = server_result {
        utils::log::error!("Error: {:?}", e)
    }
}