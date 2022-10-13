mod api;

use api::task::get_task; // Api To use to query the state of a task

use actix_web::{middleware::Logger, web::Data, App, HttpServer}; // Import from actix framework

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set logging variable that actix web reads to determine if you need logging or not
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Create HttpServer struct
    HttpServer::new(move || {
        let logger = logger::default();
        App::new().wrap(logger).service(get)
    })
    .bind(("127.0.0.1", 5050))?
    .run()
    .await
}
