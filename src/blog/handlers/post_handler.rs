use std::io::Error;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PostData {
    title: String,
    file_name: String,
    description: String,
    tags: Vec<String>,
    author: String,
    date: String,
}

fn extract_post_markdown(post_name: String, content_filename: &str) -> Result<String, Error> {
    // convert from markdown to html?
    // of course we can't find "content.md" here because we aren't in the correcty
    // directory
    match std::fs::read_to_string(format!("./posts/{}/{}", post_name, content_filename)){
        Ok(content) => return Ok(content),
        Err(e) => {
            println!("{}", e); // replace me with log pls pls pls
            println!("{}, {}", post_name, content_filename); // replace me with log pls pls pls
            return Err(e);
        }
    }
}

fn extract_post_metadata(file_name: String) -> Result<PostData, Error> {
    match std::fs::read_to_string(file_name) {
        Ok(data) => match toml::from_str::<PostData>(&data) {
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

    match extract_post_metadata(format!("posts/{}/post_data.toml", post_name)) {
        Ok(post_meta_data) => {
            context.insert("meta_data", &post_meta_data);
        }
        Err(e) => {
            println!("{}", e);
            return HttpResponse::InternalServerError().into();
        }
    }

    match context.get("meta_data") {
        Some(meta) => {
            match meta.get("file_name").and_then(|file_name| file_name.as_str()) {
                Some(file_name) => {
                    // we can now extract and parse our markdown
                    match extract_post_markdown(post_name, file_name) {
                        Ok(content) => {
                            context.insert("post_content", &markdown::to_html(&content));
                        }
                        // we don't have to have content. 500 are sad. fail softly
                        Err(_) => println!(""),
                    }
                }
                None => return HttpResponse::NoContent().into(),
            }
        }
        None => return HttpResponse::NoContent().into(),
    }

    match templ.render("post.html", &context) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(e) => {
            println!("{}", e); // we really should log instead of stdout
            HttpResponse::InternalServerError().into()
        }
    }
}
