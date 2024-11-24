use std::io::Error;

use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::post_handler::{extract_post_markdown_from_path, extract_post_metadata, serialize_html_from_markdown, PostMetaData};


#[derive(Serialize, Deserialize)]
pub struct HomeData {
    title: String,
    file_name: String,
    description: String,
    author: String,
}

fn find_all_posts_by_date_sorted() -> Result<Vec<(String, String)>, Error> {
    let mut t = ignore::types::TypesBuilder::new();
    t.add_defaults();
    let toml = match t.select("toml").build() {
        Ok(t) => t,
        Err(_) => {
            return Err(Error::new(std::io::ErrorKind::Other, "Could not find markdown type"));
        }
    };

    let walk_builder = ignore::WalkBuilder::new("./posts").types(toml).build();
    let mut posts_by_date: Vec<(String, String)> = Vec::new();
    for metadata in walk_builder {
        match metadata {
            Ok(entry) => {
                if entry.path().is_file() {
                    let post_path = entry.path();
                    let metadata: PostMetaData = extract_post_metadata(&post_path)?;
                    let content_path = post_path.parent().unwrap().join(&metadata.file_name);
                    match extract_post_markdown_from_path(&content_path) {
                        Ok(markdown) => {
                            posts_by_date.push((metadata.date.clone(), serialize_html_from_markdown(markdown)?));
                        }
                        Err(e) => {
                            println!("we are failing here {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    posts_by_date.sort_by(|a, b| b.0.cmp(&a.0));
    return Ok(posts_by_date);
}

#[get("/")]
pub async fn index(templates: web::Data<tera::Tera>) -> impl Responder {
    let mut ter_context = tera::Context::new();

    match find_all_posts_by_date_sorted() {
        Ok(posts) => {
            ter_context.insert("all_posts", &posts);
        }
        Err(e) => {
            println!("FAILED TO INSERT ALL POSTS: {}", e);  // replace with logging pls pls pls
        }
    }

    match templates.render("home.html", &ter_context) {
        Ok(page) => HttpResponse::Ok().content_type("text/html").body(page),
        Err(e) => {
            println!("Error rendering template: {:?}", e);
            println!("PATH: {}", std::env::current_dir().unwrap().display());
            HttpResponse::InternalServerError().finish()
        },
    }

}
