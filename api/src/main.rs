mod web;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    web::server::start().await
}
