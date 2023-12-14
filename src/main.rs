//! The main program of my website.
pub mod articles;
pub mod admin;
use crate::articles::*;
use crate::admin::*;
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
    "/projects" => projects_page(HTML, "../webpages/projects.html"),
    "/about" => about_page(HTML, "../webpages/about.html"),
    "/services" => services_page(HTML, "../webpages/services.html"),
    "/github-mark.svg" => github_mark(SVG, "../webpages/github-mark.svg"),
    "/article-open/<_>" => article(HTML, "../webpages/article.html"),
    "/style.css" => style(CSS, "../webpages/style.css"),
    "/logo" => logo_svg(SVG, "../webpages/logo.svg"),
}

#[get("/favicon.ico")]
fn favicon() -> &'static [u8] {
     include_bytes!("../webpages/logo.ico")
}

// The main function of the website.
#[launch]
async fn rocket() -> _ {
    load_article();

    rocket::build()
        .configure(Config {
            log_level: rocket::config::LogLevel::Critical,
            port: 80,
            ..Default::default()
        })
        .mount("/", raw_routes())
        .mount("/", routes![favicon])
        .mount(
            "/article",
            routes![
                get_images,
                new_article,
                get_article_list,
                get_article,
                get_minia_article,
                get_random_article
            ],
        ).mount("/admin", routes![login_admin, admin_main, new_article_page])
}
