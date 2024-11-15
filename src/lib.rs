
pub mod blog {
    pub mod entry;
    pub mod handlers {
        pub mod home_handler;
        pub mod post_handler;
    }
}


use tera::Tera;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        // where is this relative to?
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}
