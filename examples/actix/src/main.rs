use actix_web::{get, HttpResponse, App, HttpServer};
use color_eyre::Result;

use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    std::env::set_var("RUST_LOG", "info");

    // Load .env file
    //dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()) // load env RUST_LOG 
    .init();

    let host = "localhost";
    let port = "8000";

    info!("Starting server at http://{}:{}/", &host, &port);

    HttpServer::new(move || {
        App::new()
            .service(health)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    Ok(())
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

