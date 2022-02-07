use actix_web::{get, post, web, App, HttpResponse, HttpServer};

// Sync
use std::sync::Arc;
use tokio::sync::Mutex;
// Error handling
use color_eyre::Result;
// Log handling
use tracing::{info, trace};

#[actix_rt::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    std::env::set_var("RUST_LOG", "info,actix_example=trace");

    // Load .env file
    //dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()) // load env RUST_LOG
        .init();

    let host = "localhost";
    let port = "8000";

    info!("Starting server at http://{}:{}/", &host, &port);
    info!(
        "API: 'curl -X POST http://{}:{}/api/v1/service/start'",
        &host, &port
    );
    info!(
        "API: 'curl -X POST http://{}:{}/api/v1/service/stop'",
        &host, &port
    );

    // Async parallel runner

    let service_manager = aprun::ServiceManager::default();
    let runner = aprun::ServiceRunner::new(Arc::new(Mutex::new(service_manager)));
    let control = aprun::RunnerController::new(&runner);
    let control_stop = control.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(control.clone())
            .service(web::scope("/api/v1/service").service(stop).service(start))
            .service(health)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    // Stopping runner
    info!("Stopping runner");
    control_stop.lock().await.stop();

    Ok(())
}

#[post("/stop")]
pub async fn stop(
    control: web::Data<Arc<Mutex<aprun::RunnerController>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let control = control.lock().await;
    trace!("Stopping runner");
    control.stop();
    Ok(HttpResponse::Ok().body("control stopped"))
}

#[post("/start")]
pub async fn start(
    control: web::Data<Arc<Mutex<aprun::RunnerController>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let control = control.lock().await;
    trace!("Starting runner");
    control.run().await;
    Ok(HttpResponse::Ok().body("control started"))
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
