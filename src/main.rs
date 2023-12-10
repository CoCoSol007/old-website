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
    "/" => main_page(HTML, "../webpages/home.html"),
    "/home.html" => wave(HTML, "../webpages/home.html"),
    "/about.html" => aboute_me_page(HTML, "../webpages/about.html"),
    "/articles.html" => articles_page(HTML, "../webpages/articles.html"),
    "/projects.html" => projects_page(HTML, "../webpages/projects.html"),
    "/add_article.html" => add_article_page(HTML, "../webpages/add_article.html"),
    "/style.css" => style(CSS, "../webpages/style.css"),
    "/github-mark.png" => github_mark(SVG, "../webpages/github-mark.svg"),
    "/contact_me.png" => contact_me_mark(SVG, "../webpages/contact_me.svg"),
    "/article/<_>" => article_page(HTML, "../webpages/article.html"),
}

/// a fonction to give the icon of the webpage.
#[get("/favicon.ico")]
const fn favicon() -> (ContentType, &'static [u8]) {
    let ico_data: &'static [u8] = include_bytes!("../webpages/favicon.ico");
    let content_type = ContentType::Icon;

    (content_type, ico_data)
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
        .mount("/", routes![favicon])
        .mount("/article", routes![new_article, get_article_list, get_article, get_minia_article, get_random_article])
}
