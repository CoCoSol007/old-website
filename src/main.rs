//! The main program of my website.
use rand::Rng;
use rocket::catch;
use rocket::catchers;
use serde::{Deserialize, Serialize};
pub mod admin;
pub mod articles;
use crate::admin::*;
use crate::articles::*;
use once_cell::sync::Lazy;
use rocket::response::Redirect;
use rocket::{
    form::Form, get, http::ContentType, launch, post, routes, serde::uuid::Uuid, Config, FromForm,
    Route,
};
use std::net::IpAddr;
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

/// a struct to store messages.
#[derive(Serialize, Deserialize, Clone, Debug, FromForm)]
struct Msg {
    name: String,
    email: String,
    message: String,
}

/// a fonction to handle 404 for custom 404 error page
#[catch(404)]
fn handle_404() -> (ContentType, &'static str) {
    (ContentType::HTML, include_str!("../webpages/404.html"))
}

/// a fonction to receive a message
#[post("/send_msg", format = "multipart/form-data", data = "<data>")]
fn receive_msg(data: Form<Msg>) -> Redirect {
    // TODO
    println!("new msg : {:#?}", data);

    Redirect::to("/")
}

/// a fonction to serve the favicon
#[get("/favicon.ico")]
fn favicon() -> &'static [u8] {
    include_bytes!("../webpages/logo.ico")
}

/// a fonction to generate a secret key
fn generate_secret_key() -> String {
    // Utiliser la bibliothèque rand pour générer des octets aléatoires
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

    // Convertir les octets en une chaîne hexadécimale
    let hex_string: String = random_bytes.iter().map(|b| format!("{:02x}", b)).collect();

    hex_string
}

// The main function of the website.
#[launch]
async fn rocket() -> _ {
    // load articles
    load_article();

    // generate a secret key and figment the rocket server
    let figment = Config::figment()
        .merge(("secret_key", generate_secret_key()))
        .merge(("port", 80))
        .merge(("worker_count", 4))
        .merge(("log_level", rocket::config::LogLevel::Critical))
        .merge(("address", IpAddr::from([0, 0, 0, 0])));

    // create a config
    let config = Config::from(figment);

    // launch the server with different routes
    rocket::build()
        // apply the config
        .configure(config)
        // for 404 error
        .register("/", catchers![handle_404])
        // for static files
        .mount("/", raw_routes())
        .mount("/", routes![favicon, receive_msg])
        // for articles
        .mount(
            "/article",
            routes![
                get_images,
                new_article,
                get_article_list,
                get_article,
                get_minia_article,
                get_random_article,
                delete_article
            ],
        )
        // for admin
        .mount(
            "/admin",
            routes![
                login_admin,
                admin_main,
                new_article_page,
                modify_article_page
            ],
        )
}
