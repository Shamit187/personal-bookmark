use actix_web::{get, web, Result, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Info {
    book_id: u32,
    chapter: u32,
    subchapter: u32,
}

/// extract path info using serde
#[get("/notes/{book_id}/{chapter}/{subchapter}")] // <- define path parameters
async fn index(info: web::Path<Info>) -> Result<HttpResponse> {
    // Generate HTML content dynamically
    let html_content = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Notes</title>
        </head>
        <body>
            <h1>Welcome to chapter {} of book {} subchapter {}</h1>
        </body>
        </html>
        "#,
        info.chapter, info.book_id, info.subchapter
    );

    Ok(HttpResponse::Ok().content_type("text/html").body(html_content))
}

pub fn note_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
