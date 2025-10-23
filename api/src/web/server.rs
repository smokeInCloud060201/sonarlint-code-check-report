use actix_cors::Cors;
use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use listenfd::ListenFd;
use std::env;
use std::sync::Arc;
use tracing::info;
use crate::config::logger;
use crate::sonarqube::{SonarQubeService, models::SonarQubeConfig};

pub async fn start() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    logger::init();

    let server_host = env::var("SERVER_HOST").expect("HOST not set in .env file");
    let server_port = env::var("SERVER_PORT").expect("PORT not set in .env file");
    let server_url = format!("{server_host}:{server_port}");

    // Initialize SonarQube service
    let sonar_config = SonarQubeConfig {
        base_url: env::var("SONARQUBE_URL").unwrap_or_else(|_| "http://localhost:9000".to_string()),
        username: env::var("SONARQUBE_USERNAME").unwrap_or_else(|_| "admin".to_string()),
        password: env::var("SONARQUBE_PASSWORD").unwrap_or_else(|_| "admin".to_string()),
        timeout_seconds: env::var("SONARQUBE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30),
    };

    let sonar_service = match SonarQubeService::new(sonar_config) {
        Ok(service) => {
            info!("SonarQube service initialized successfully");
            Arc::new(service)
        }
        Err(e) => {
            tracing::error!("Failed to initialize SonarQube service: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to initialize SonarQube service: {}", e),
            ));
        }
    };

    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_method()
            .allow_any_origin()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(sonar_service.clone()))
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
    cfg.service(
        web::scope("/api/sonarqube")
            .service(
                web::resource("/projects")
                    .route(web::post().to(crate::sonarqube::handlers::create_project))
            )
            .service(
                web::resource("/projects/{project_key}")
                    .route(web::get().to(crate::sonarqube::handlers::get_project_info))
                    .route(web::delete().to(crate::sonarqube::handlers::delete_project))
            )
            .service(
                web::resource("/projects/{project_key}/exists")
                    .route(web::get().to(crate::sonarqube::handlers::check_project_exists))
            )
            .service(
                web::resource("/issues")
                    .route(web::post().to(crate::sonarqube::handlers::get_project_issues))
            )
            .service(
                web::resource("/issues/all")
                    .route(web::post().to(crate::sonarqube::handlers::get_all_project_issues))
            )
            .service(
                web::resource("/health")
                    .route(web::get().to(crate::sonarqube::handlers::health_check))
            )
            .service(
                web::resource("/version")
                    .route(web::get().to(crate::sonarqube::handlers::get_server_version))
            )
    );
}
