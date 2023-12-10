//! The main program of my website.
pub mod articles;
use crate::articles::*;
use once_cell::sync::Lazy;
use rocket::{get, http::ContentType, launch, routes, serde::uuid::Uuid, Config, Route};
use std::{collections::HashMap, sync::RwLock};

/// a struct to store articles.
static ARTICLES: Lazy<RwLock<HashMap<Uuid, Article>>> = Lazy::new(|| RwLock::new(HashMap::new()));

/// a macro to define a route.
macro_rules! raw_files {
    {$($route:expr => $name:ident($content_type:ident, $path:expr),)*} => {
        $(
            #[doc = "a fonction to serve the file"]
            #[get($route)]
            const fn $name() -> (ContentType, &'static str) {
                (ContentType::$content_type, include_str!($path))
            }
        )*

        fn raw_routes() -> Vec<Route> {
            routes![$($name),*]
        }
    };
}

raw_files! {
    "/" => main_page(HTML, "../webpages/main.html"),
}

// The main function of the website.
#[launch]
async fn rocket() -> _ {
    load_article();

    rocket::build()
        .configure(Config {
            // pas de output
            log_level: rocket::config::LogLevel::Critical,
            port: 80,
            ..Default::default()
        })
        .mount("/", raw_routes())
        .mount("/article", routes![new_article, get_article_list, get_article, get_minia_article, get_random_article])
}
