use std::{io::Error, path::Path};

use actix_web::{web, HttpResponse};
use pulldown_cmark::{Event, Tag};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PostMetaData {
    pub title: String,
    pub file_name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub author: String,
    pub date: String,
}

pub fn serialize_html_from_markdown(markdown: String) -> Result<String, Error> {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    let parser = pulldown_cmark::Parser::new_ext(&markdown, options)
        .map(|event| match event {
            Event::Start(Tag::List(None)) => {
                Event::Html("<ul class=\"list-square list-inside\">".into())
            }
            _ => event,
        });

    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    return Ok(html_output);
}

pub fn extract_post_markdown_from_path(filepath: &Path) -> Result<String, Error> {
    match std::fs::read_to_string(filepath) {
        Ok(content) => return Ok(content),
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn extract_post_markdown(post_name: String, content_filename: &str) -> Result<String, Error> {
    match std::fs::read_to_string(format!("./posts/{}/{}", post_name, content_filename)) {
        Ok(content) => return Ok(content),
        Err(e) => {
            println!("{}", e); // replace me with log pls pls pls
            println!("{}, {}", post_name, content_filename); // replace me with log pls pls pls
            return Err(e);
        }
    }
}

pub fn extract_post_metadata(file_name: &Path) -> Result<PostMetaData, Error> {
    match std::fs::read_to_string(file_name) {
        Ok(data) => match toml::from_str::<PostMetaData>(&data) {
            Ok(post_data) => {
                return Ok(post_data);
            }
            Err(e) => {
                println!("{}", e); // replace me with log pls pls pls
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Could not extract expected fields from toml",
                ));
            }
        },
        Err(e) => {
            return Err(e);
        }
    }
}

#[actix_web::get("/posts/{post_name}")]
pub async fn get_post(templ: web::Data<tera::Tera>, post_name: web::Path<String>) -> HttpResponse {
    let post_name = post_name.into_inner();
    let mut context = tera::Context::new();

    match extract_post_metadata(Path::new(&format!("posts/{}/post_data.toml", post_name))) {
        Ok(post_meta_data) => {
            context.insert("meta_data", &post_meta_data);
        }
        Err(e) => {
            println!("{}", e);
            return HttpResponse::InternalServerError().into();
        }
    }

    if let Some(meta) = context.get("meta_data") {
        if let Some(file_name) = meta.get("file_name").and_then(|fnm| fnm.as_str()) {
            match extract_post_markdown(post_name, file_name) {
                Ok(content) => match serialize_html_from_markdown(content) {
                    Ok(html) => {
                        context.insert("post_content", &html);
                    }
                    Err(e) => {
                        println!("{}", e);
                        return HttpResponse::InternalServerError().into();
                    }
                },
                Err(_) => {
                    // Handle the case where markdown extraction failed
                    println!("Failed to extract post markdown for file: {}", file_name);
                    return HttpResponse::InternalServerError().into();
                }
            }
        } else {
            return HttpResponse::NoContent().into();
        }
    } else {
        return HttpResponse::NoContent().into();
    }

    match templ.render("post.html", &context) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(e) => {
            println!("{}", e); // we really should log instead of stdout
            HttpResponse::InternalServerError().into()
        }
    }
}
