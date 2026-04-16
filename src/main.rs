use actix_web::{web, App, HttpServer, middleware};

mod routes;
mod storage;
mod utils;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default()) // Logging middleware for better observability
            .wrap(auth::AuthMiddleware {}) // Custom authentication middleware
            .service(routes::tokenize)
            .service(routes::detokenize)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
