use std::net::TcpListener;

use actix_web::{dev::Server, middleware, web, App, HttpResponse, HttpServer};



pub fn start_entry(listener: TcpListener) -> Result<Server, std::io::Error>{

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(HttpResponse::Ok))
    })
    .listen(listener)?
    .run();

    return Ok(server);

}
