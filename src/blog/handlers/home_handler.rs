use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct HomeData {
    title: String,
    file_name: String,
    description: String,
    author: String,
}

#[get("/")]
pub async fn index(templates: web::Data<tera::Tera>) -> impl Responder {
    let mut ter_context = tera::Context::new();

    let data = HomeData {
        title: "Welcome to the Blog".to_string(),
        author: "John Doe".to_string(),
        file_name: "home_handler.rs".to_string(),
        description: "This is the home page of the blog".to_string(),
    };

    // what are youuuuuuu
    ter_context.insert("my_content", &data);

    // we can't find the path to the template likely.
    match templates.render("home.html", &ter_context) {
        Ok(page) => HttpResponse::Ok().content_type("text/html").body(page),
        Err(e) => {
            println!("Error rendering template: {:?}", e);
            println!("PATH: {}", std::env::current_dir().unwrap().display());
            HttpResponse::InternalServerError().finish()
        },
    }

}
