mod web;
mod config;
mod sonarqube;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    web::server::start().await
}
