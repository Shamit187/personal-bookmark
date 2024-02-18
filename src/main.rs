use actix_files as fs;
use actix_web::{App, HttpServer};
use std::collections::HashMap;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut directories = HashMap::new();
    directories.insert("/", "./static");
    directories.insert("/login", "./static/login");

    HttpServer::new(move || {
        let mut app = App::new();

        // add api routes
        app = app.configure(api::api_routes);

        for (path, dir) in &directories {
            app = app.service(fs::Files::new(path, dir).index_file("index.html"));
        }
        
        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
