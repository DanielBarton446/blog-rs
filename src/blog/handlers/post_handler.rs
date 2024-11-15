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



fn extract_post_data(file_name: String) -> Option<PostData> {
    match std::fs::read_to_string(file_name) {
        Ok(data) => {
            match toml::from_str::<PostData>(&data) {
                Ok(_post_data) => {
                    let post_data: PostData = toml::from_str(&data).unwrap();
                    return Some(post_data);
                }
                Err(e) => {
                    println!("{}", e);
                    return None;
                }
            }
        },
        Err(_) => {
            return None;
        }
    }
}



#[actix_web::get("/posts/{post_name}")]
pub async fn get_post(
    templ: web::Data<tera::Tera>,
    post_name: web::Path<String>) -> HttpResponse {
    let post_name = post_name.into_inner();

    let mut context = tera::Context::new(); 


    match extract_post_data(format!("posts/{}/post_data.toml", post_name)) {
        Some(post_meta_data) => {
            context.insert("meta_data", &post_meta_data);
            match templ.render("post.html", &context) {
                Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
                Err(e) => {
                    println!("{}", e);  // we really should log instead of stdout
                    HttpResponse::InternalServerError().body("Something went terribly wrong")},
            }
        },
        None => HttpResponse::NotFound().body("Not found!")

    }

}
