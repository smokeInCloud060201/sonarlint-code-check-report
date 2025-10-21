use actix_cors::Cors;
use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use listenfd::ListenFd;
use std::env;
use tracing::info;
use crate::config::logger;

pub async fn start() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    logger::init();

    let server_host = env::var("SERVER_HOST").expect("HOST not set in .env file");
    let server_port = env::var("SERVER_PORT").expect("PORT not set in .env file");
    let server_url = format!("{server_host}:{server_port}");


    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_method()
            .allow_any_origin()
            .allow_any_header();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .configure(init_config)
    });

    let mut listen_fd = ListenFd::from_env();
    server = if let Some(listener) = listen_fd.take_tcp_listener(0)? {
        server.listen(listener)?
    } else {
        server.bind(&server_url)?
    };

    info!("Starting server at {server_url}");
    server.run().await
}

fn init_config(cfg: &mut web::ServiceConfig) {

}
