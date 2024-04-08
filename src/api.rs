use actix_web::{web, HttpResponse};
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct BookItem {
    book_id: u32,
    book_title: String,
    author: String,
    genre: String,
    content: Vec<ContentKey>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentKey {
    title: String,
    content_id: u32,
}

fn list_to_html(list: Vec<BookItem>) -> String {
    let mut html = String::from("<div class='flex flex-col'>");
    for book in list {
        html.push_str(&format!(
            "<div><a href=\"/notes/{}/0\">{} : {}</a></div>",
            book.book_id, book.book_title, book.author
        ));
    }
    html.push_str("</div>");

    html
}

async fn query_db(genre: String) -> Vec<BookItem> {
    let mongodb_uri = env::var("MONGOURI").expect("MONGODB_URI must be set");
    let client = mongodb::Client::with_uri_str(&mongodb_uri).await.unwrap();
    let db = client.database("personal-bookmark");
    let collection = db.collection::<BookItem>("books");

    println!("Querying the database for genre: {}", genre);

    let mut cursor = collection
        .find(doc! {"genre" : genre}, None)
        .await
        .expect("Error finding documents");

    let mut books = Vec::new();
    while let Some(book) = cursor
        .try_next()
        .await
        .expect("Error getting next document")
    {
        books.push(book);
    }
    books
}

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

/*
genre mapping
"Discrete Mathematics"
"Continuous Mathematics"
"Casual Technical"
"Quantum Computing"
"Information and Communication"
"Physics"
"Chemistry"
"Artificial Intelligence"
"Biology"
"Algorithm"
"Japanese Literature"
"Casual Non-Literature"
"Manga"
"Bangla Literature"
"English Literature"
"Scientific Papers"
"Others"
*/

async fn discrete_math() -> HttpResponse {
    let list = query_db(String::from("Discrete Mathematics")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn continuous_math() -> HttpResponse {
    let list = query_db(String::from("Continuous Mathematics")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn casual_technical() -> HttpResponse {
    let list = query_db(String::from("Casual Technical")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn quantum_computing() -> HttpResponse {
    let list = query_db(String::from("Quantum Computing")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn inf_communication() -> HttpResponse {
    let list = query_db(String::from("Information and Communication")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn physics() -> HttpResponse {
    let list = query_db(String::from("Physics")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn chemistry() -> HttpResponse {
    let list = query_db(String::from("Chemistry")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn ai() -> HttpResponse {
    let list = query_db(String::from("Artificial Intelligence")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn bio() -> HttpResponse {
    let list = query_db(String::from("Biology")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn algo() -> HttpResponse {
    let list = query_db(String::from("Algorithm")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn japanese_lit() -> HttpResponse {
    let list = query_db(String::from("Japanese Literature")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn casual_non_lit() -> HttpResponse {
    let list = query_db(String::from("Casual Non-Literature")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn manga() -> HttpResponse {
    let list = query_db(String::from("Manga")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn bangla_lit() -> HttpResponse {
    let list = query_db(String::from("Bangla Literature")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn english_lit() -> HttpResponse {
    let list = query_db(String::from("English Literature")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn research() -> HttpResponse {
    let list = query_db(String::from("Scientific Papers")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

async fn others() -> HttpResponse {
    let list = query_db(String::from("Others")).await;
    HttpResponse::Ok().body(list_to_html(list))
}

// Create a function to configure the API routes
pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/discrete-math", web::get().to(discrete_math))
            .route("/continuous-math", web::get().to(continuous_math))
            .route("/casual-technical", web::get().to(casual_technical))
            .route("/quantum-computing", web::get().to(quantum_computing))
            .route("/inf-communication", web::get().to(inf_communication))
            .route("/physics", web::get().to(physics))
            .route("/chemistry", web::get().to(chemistry))
            .route("/ai", web::get().to(ai))
            .route("/bio", web::get().to(bio))
            .route("/algo", web::get().to(algo))
            .route("/japanese-literature", web::get().to(japanese_lit))
            .route("/casual-non-lit", web::get().to(casual_non_lit))
            .route("/manga", web::get().to(manga))
            .route("/bangla", web::get().to(bangla_lit))
            .route("/english", web::get().to(english_lit))
            .route("/research", web::get().to(research))
            .route("/others", web::get().to(others)),
    );
}
