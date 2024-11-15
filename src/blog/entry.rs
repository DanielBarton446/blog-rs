use std::net::TcpListener;

use actix_web::{dev::Server, middleware, web, App, HttpResponse, HttpServer};

use crate::TEMPLATES;
use crate::blog::handlers;

pub fn start_blog(listener: TcpListener) -> Result<Server, std::io::Error>{

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(TEMPLATES.clone()))
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(handlers::home_handler::index)
            .service(handlers::post_handler::get_post)
    })
    .listen(listener)?
    .run();

    return Ok(server);

}
