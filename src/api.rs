use actix_web::{web, HttpResponse};

// Define your API handlers
async fn demo_api1() -> HttpResponse {
    HttpResponse::Ok().body("Demo API 1")
}

async fn demo_api2() -> HttpResponse {
    HttpResponse::Ok().body("Demo API 2")
}

// Create a function to configure the API routes
pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/demo1", web::get().to(demo_api1))
            .route("/demo2", web::get().to(demo_api2))
    );
}