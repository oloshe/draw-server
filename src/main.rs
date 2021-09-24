use app;

#[actix_web::main]
async fn main() {
    utils::logger::init_logger();
    app::create_app().await;
}
