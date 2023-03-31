use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{get, middleware, HttpResponse, Responder};
use actix_web::{web, App, HttpServer};
use log::{debug, info};
use serde::Serialize;

use crate::config::ServerConfig;
use crate::generator::Generator;
use crate::{logger, worker};

pub async fn run(config: ServerConfig) -> std::io::Result<()> {
    // Set the default log level
    logger::init(&config.log);

    // Output circle banner
    info!("Starting server...");

    // Initialize server shared state
    let worker = Arc::new(worker::LeaseWorker::new());
    let worker_id = worker.start(&config.get_database_addr()).await.unwrap();
    let generator = Arc::new(Generator::new(worker_id.into(), 0));

    // Start Http server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(Data::new(generator.clone()))
            .configure(routes)
    })
    .bind((config.host.clone(), config.port))
    .expect("Unable to resolves socket address and bind server to listener.")
    .disable_signals()
    .run();

    let server_handle = server.handle();
    let server_task = tokio::spawn(async move {
        info!("Server running at http://{}:{}", config.host, config.port);
        server.await
    });

    let shutdown_task = tokio::spawn(async move {
        // Listen for ctrl-c
        tokio::signal::ctrl_c().await.unwrap();
        info!("Global shutdown has been initiated...");

        server_handle.stop(true).await;
        debug!("HTTP server shutdown completed...");
    });

    let _ = tokio::try_join!(server_task, shutdown_task).expect("unable to join tasks");

    Ok(())
}

fn routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("api/v1/snowflake").configure(configure));
}

fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(next_id);
}

#[get("next/{count}")]
async fn next_id(p: web::Path<i32>, generator: Data<Arc<Generator>>) -> impl Responder {
    let count = p.into_inner();
    let mut ids = Vec::new();

    for _ in 0..count {
        ids.push(generator.next_id().unwrap());
    }

    HttpResponse::Ok().json(NextIdResponse { ids })
}

#[derive(Serialize)]
struct NextIdResponse {
    ids: Vec<i64>,
}
