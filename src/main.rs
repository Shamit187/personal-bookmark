use actix_web::{web, App, HttpServer};

mod app_scope;

// trying out rust's web scope

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/app")
                .route("/1", web::get().to(app_scope::app1))
                .route("/2", web::get().to(app_scope::app2))
                .route("/3", web::get().to(app_scope::app3)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
