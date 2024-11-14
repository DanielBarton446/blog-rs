use std::net::TcpListener;
use blog_rs::blog::entry;





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();


    let listener = TcpListener::bind("127.0.0.1:8080")?;

    entry::start_blog(listener)?.await?;

    Ok(())
}
