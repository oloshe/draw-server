use app;

#[actix_web::main]
async fn main() {
    utils::init_logger();
    app::create_app().await;
}
