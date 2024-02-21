use actix_web::{App, HttpServer};
use std::env;
use dotenv::dotenv;

mod api;
mod file; // Import the file server module
mod note;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read port from environment variable
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);

    // Connect to the database
    // let mongodb_uri = env::var("MONGOURI").expect("MONGODB_URI must be set");
    // let client = mongodb::Client::with_uri_str(&mongodb_uri).await.unwrap();
    // let collection = client.database("personal-bookmark").collection::<note::TableOfContent>("toc");
    // let mut cursor = collection.find(doc! {"book_id" : 10}, None).await.unwrap();
    // let table_of_content = cursor.try_next().await.unwrap().unwrap();
    // println!("Title: {}", table_of_content.title);

    // Start the server
    println!("Starting server on: {}", &address);
    HttpServer::new(|| {
        let mut app = App::new();

        // Configure API routes first
        app = app.configure(api::api_routes);

        // Configure note routes next
        app = app.configure(note::note_routes);

        // Configure file server routes last
        app = app.configure(file::frontend_routes);

        app
    })
    .bind(address)?
    .run()
    .await
}

