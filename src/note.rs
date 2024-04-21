use actix_web::{get, web, HttpResponse, Result};
use actix_files;
use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use std::time::Instant;

use crate::markdown_to_html::start;

#[derive(Serialize, Deserialize, Debug)]
struct BookItem {
    book_id: u32,
    book_title: String,
    author: String,
    genre: String,
    total_content: u32,
    completed_content: u32,
    content: Vec<ContentKey>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentKey {
    title: String,
    content_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentItem {
    title: String,
    book_id : u32,
    content_id : u32,
    markdown_content : String,
    date : String,
}

#[derive(Deserialize, Debug)]
struct Info {
    book_id: u32,
    content_id: u32,
}

fn generate_table_of_content(book: BookItem) -> String {
    let mut toc_html = String::from("");
    for content in &book.content {
        toc_html.push_str(&format!(
            "<a class=\"hover:no-underline text-neutral-800 dark:text-neutral-400 hover:pl-4\"href=\"/notes/{}/{}\">{}</a>",
            book.book_id, content.content_id, content.title
        ));
    }

    toc_html
}

/// extract path info using serde
#[get("/notes/{book_id}/{content_id}")] // <- define path parameters
async fn index(info: web::Path<Info>) -> Result<HttpResponse> {
    // Read the contents of the files
    // todo: add styles from markdown to html
    let note_top_html =
        fs::read_to_string("template/note_top.txt").expect("Unable to read note_top.txt");
    let note_middle_html =
        fs::read_to_string("template/note_middle.txt").expect("Unable to read note_middle.txt");
    let note_bottom_content =
        fs::read_to_string("template/note_bottom.txt").expect("Unable to read note_bottom.txt");
        
    // Query the database using book_id, content_id
    // todo: need to change query
    println!(
        "Querying the database for book_id: {} content_id: {}",
        info.book_id, info.content_id
    );

    // Read from database
    let mongodb_uri = env::var("MONGOURI").expect("MONGODB_URI must be set");
    let client = mongodb::Client::with_uri_str(&mongodb_uri).await.unwrap();
    let db = client.database("personal-bookmark");

    // fetch table of content
    let collection = db.collection::<BookItem>("books");
    let mut cursor = collection
        .find(doc! {"book_id" : info.book_id}, None)
        .await
        .unwrap();
    let book = match cursor.try_next().await {
        Ok(Some(book)) => book,
        Ok(None) => {
            println!("No book found for book_id: {}", info.book_id);
            BookItem {
                book_id: info.book_id,
                book_title: String::from("Sorry, no book found 😔"),
                author: String::from(""),
                genre: String::from(""),
                total_content: 0,
                completed_content: 0,
                content: vec![ContentKey {
                    title: String::from(""),
                    content_id: 0,
                }],
            }
        }
        Err(e) => {
            println!("Error querying the database: {}", e);
            BookItem {
                book_id: info.book_id.clone(),
                book_title: String::from("Sorry, no book found 😔"),
                author: String::from(""),
                genre: String::from(""),
                total_content: 0,
                completed_content: 0,
                content: vec![ContentKey {
                    title: String::from(""),
                    content_id: 0,
                }],
            }
        }
    };
    let toc_html = generate_table_of_content(book);
    
    // if content_id is 0, return the table of content
    if info.content_id == 0 {
        let html_content = format!(
            "{} {} {} {}",
            note_top_html, toc_html, note_middle_html, note_bottom_content
        );
        return Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(html_content));
    }

    // fetch actual note content
    let collection = db.collection::<ContentItem>("contents");
    let mut cursor = collection
        .find(doc! {"book_id" : info.book_id.clone(), "content_id" : info.content_id.clone()}, None)
        .await
        .unwrap();
    let note = match cursor.try_next().await {
        Ok(Some(note)) => note,
        Ok(None) => {
            println!(
                "No note found for book_id: {} content_id: {}",
                info.book_id, info.content_id
            );
            ContentItem {
                title: String::from("Sorry, no note found 😔"),
                book_id: info.book_id,
                content_id: info.content_id,
                markdown_content: String::from(""),
                date: String::from(""),
            }
        }
        Err(e) => {
            println!("Error querying the database: {}", e);
            ContentItem {
                title: String::from("Sorry, no note found 😔"),
                book_id: info.book_id,
                content_id: info.content_id,
                markdown_content: String::from(""),
                date: String::from(""),
            }
        }
    };
    let start_time = Instant::now();
    let note_html = start(note.markdown_content);
    let elapsed_html = format!("<div class=\"author\">Processing time: {:?}</div>", start_time.elapsed());
    // Generate HTML content dynamically
    let html_content = format!(
        "{} {} {} {} {} {}",
        note_top_html, toc_html, note_middle_html, note_html, elapsed_html, note_bottom_content
    );

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content))
}

#[get("/favicon.ico")]
async fn favicon() -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("static/favicon.png")?)
}


pub fn note_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(favicon)
    .service(index);
}
