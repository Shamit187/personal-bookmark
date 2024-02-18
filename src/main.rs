use actix_web::{App, HttpServer};

mod api;
mod file; // Import the file server module

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let mut app = App::new();

        // Configure API routes
        app = app.configure(api::api_routes);

        // Configure file server routes
        app = app.configure(file::frontend_routes);

        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
