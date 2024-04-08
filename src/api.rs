use actix_web::{web, HttpResponse};

// needed apis
// [
//     "/api/discrete-math",
//     "/api/continuous-math",
//     "/api/casual-technical",
//     "/api/quantum-computing",
//     "/api/inf-communication",
//     "/api/physics",
//     "/api/chemistry",
//     "/api/ai",
//     "/api/bio",
//     "/api/algo",
//     "/api/japanese-literature",
//     "/api/casual-non-lit",
//     "/api/manga",
//     "/api/bangla",
//     "/api/english",
//     "/api/research",
//     "/api/others"
// ]

async fn discrete_math() -> HttpResponse {
    HttpResponse::Ok().body(r"
    <div class='flex flex-col'>
        <div>Discrete Mathematics and Its Applications</div>
        <div>Discrete mathematics with applications</div>
        <div>Elementary Number Theory</div>
    </div>
    ")
}

async fn continuous_math() -> HttpResponse {
    HttpResponse::Ok().body(r"
    <div class='flex flex-col'>
        <div>Calculus</div>
        <div>Linear Algebra</div>
        <div>Ordinary Differential Equations</div>
        <div>Partial Differential Equations</div>
    </div>
    ")
}

async fn casual_technical() -> HttpResponse {
    HttpResponse::Ok().body(r"
    <div class='flex flex-col'>
        <div>Cracking the coding interview</div>
        <div>The Pragmatic Programmer</div>
        <div>Design Patterns</div>
    </div>
    ")
}


// Create a function to configure the API routes
pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/discrete-math", web::get().to(discrete_math))
            .route("/continuous-math", web::get().to(continuous_math))
            .route("/casual-technical", web::get().to(casual_technical))
    );
}