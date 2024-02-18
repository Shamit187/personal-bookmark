use actix_web::{App, HttpServer};
use std::env;

mod api;
mod file; // Import the file server module

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // read port from environment variable
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);

    HttpServer::new(|| {
        let mut app = App::new();

        // Configure API routes
        app = app.configure(api::api_routes);

        // Configure file server routes
        app = app.configure(file::frontend_routes);

        app
    })
    .bind(address)?
    .run()
    .await
}
