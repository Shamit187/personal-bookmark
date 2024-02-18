use actix_web::{HttpResponse, Responder};

pub async fn app1() -> impl Responder {
    HttpResponse::Ok().body("1 from App")
}

pub async fn app2() -> impl Responder {
    HttpResponse::Ok().body("2 from App")
}

pub async fn app3() -> impl Responder {
    HttpResponse::Ok().body("3 from App")
}
