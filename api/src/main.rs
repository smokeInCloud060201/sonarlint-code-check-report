mod web;
mod config;
mod utils;
mod sonarqube;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    web::server::start().await
}
