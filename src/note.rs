use actix_web::{get, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::env;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc};

#[derive(Deserialize, Debug)]
struct Info {
    book_id: u32,
    chapter: u32,
    subchapter: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct NoteContent {
    #[serde(rename = "type")]
    content_type: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Note {
    book_id: u32,
    chapter: u32,
    subchapter: u32,
    content: Vec<NoteContent>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TableOfContent {
    book_id: u32,
    title: String,
    chapter: Vec<Chapter>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Chapter {
    chapter_title: String,
    subchapter: Vec<SubChapter>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SubChapter {
    subchapter_title: String,
}


fn generate_note(contents: Vec<NoteContent>) -> Result<String, serde_json::Error> {
    let mut text = String::new();
    for item in contents {
        match item.content_type.as_str() {
            "head" => {
                text.push_str(&format!(
                    "<p class=\"text-center text-4xl m-2 p-2\"> {} </p>",
                    item.content
                ));
            }
            "text" => {
                text.push_str(&format!(
                    "<p class=\"text-left text-base m-2 p-2\"> {} </p>",
                    item.content
                ));
            }
            _ => {}
        }
    }
    Ok(text)
}

fn generate_table_of_content(json_str: &str) -> String {
    let data: Value = serde_json::from_str(json_str).unwrap();
    let title = data["title"].as_str().unwrap_or_default();
    let book_id = data["book_id"].as_u64().unwrap_or_default();
    let mut html = format!(
        "<h1 class=\"text-3xl p-2 dark:text-gray-200\">{}</h1>",
        title
    );

    if let Some(chapters) = data["chapter"].as_array() {
        for (chapter_idx, chapter) in chapters.iter().enumerate() {
            let chapter_title = chapter["chapter_title"].as_str().unwrap_or_default();
            let chapter_href = format!("/notes/{}/{}/0", book_id, chapter_idx);
            html.push_str(&format!(
                "<a href=\"{}\" class=\"block px-2 py-4 dark:text-gray-200 text-xl\">Chapter {}: {}</a>",
                chapter_href,
                chapter_idx + 1,
                chapter_title
            ));

            if let Some(subchapters) = chapter["subchapter"].as_array() {
                for (subchapter_idx, subchapter) in subchapters.iter().enumerate() {
                    let subchapter_title =
                        subchapter["subchapter_title"].as_str().unwrap_or_default();
                    let subchapter_href =
                        format!("/notes/{}/{}/{}", book_id, chapter_idx, subchapter_idx);
                    html.push_str(&format!(
                        "<a href=\"{}\" class=\"block p-2 dark:text-gray-200 text-base\">{}.{}: {}</a>",
                        subchapter_href,
                        chapter_idx + 1,
                        subchapter_idx + 1,
                        subchapter_title
                    ));
                }
            }
        }
    }

    html
}

/// extract path info using serde
#[get("/notes/{book_id}/{chapter}/{subchapter}")] // <- define path parameters
async fn index(_info: web::Path<Info>) -> Result<HttpResponse> {
    // Read the contents of the files
    let note_top_html =
        fs::read_to_string("template/note_top.txt").expect("Unable to read note_top.txt");
    let note_middle_html =
        fs::read_to_string("template/note_middle.txt").expect("Unable to read note_middle.txt");
    let note_bottom_content =
        fs::read_to_string("template/note_bottom.txt").expect("Unable to read note_bottom.txt");

    // Query the database using book_id, chapter, and subchapter
    println!(
        "Querying the database for book_id: {} chapter: {} subchapter: {}",
        _info.book_id, _info.chapter, _info.subchapter
    );

    // Read from database
    let mongodb_uri = env::var("MONGOURI").expect("MONGODB_URI must be set");
    let client = mongodb::Client::with_uri_str(&mongodb_uri).await.unwrap();
    let db = client.database("personal-bookmark");
    let collection = db.collection::<TableOfContent>("toc");
    let mut cursor = collection.find(doc! {}, None).await.unwrap();
    while let Some(result) = cursor.try_next().await.unwrap(){
        println!("{:?}", result);
    }

    // In future all will come from database

    let json_note = fs::read_to_string("template/note_content.json")?;
    let note: Note = serde_json::from_str(&json_note)?;
    let note_html = generate_note(note.content)?;

    let json_toc = fs::read_to_string("template/note_toc.json").unwrap();
    let toc_html = generate_table_of_content(&json_toc);

    // Generate HTML content dynamically
    let html_content = format!(
        "{} {} {} {} {}",
        note_top_html, toc_html, note_middle_html, note_html, note_bottom_content
    );

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content))
}

pub fn note_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
