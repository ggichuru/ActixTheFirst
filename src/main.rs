mod api;
mod repository;

use api::task::get_task; // Api To use to query the state of a task
use repository::ddb::DDBRepository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer}; // Import from actix framework

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set logging variable that actix web reads to determine if you need logging or not
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;

    // Create HttpServer struct
    HttpServer::new(move || {
        // Instantiate the DDBRepository
        let ddb_repo: DDBRepository = DDBRepository::init(String::from("task"), config.clone());

        let ddb_data = Data::new(ddb_repo);

        let logger = Logger::default();

        App::new().wrap(logger).app_data(ddb_data).service(get_task)
    })
    .bind(("127.0.0.1", 5050))?
    .run()
    .await
}
