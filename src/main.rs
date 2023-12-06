//! The main program of my website.

use rocket::{
    get, http::ContentType, launch, routes,Config,
    Route,
};

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
    "/style.css" => style(CSS, "../webpages/style.css"),
    "/github-mark.png" => github_mark(SVG, "../webpages/github-mark.svg"),
    "/contact_me.png" => contact_me_mark(SVG, "../webpages/contact_me.svg"),
}

#[get("/favicon.ico")]
const fn favicon() -> (ContentType, &'static [u8]) {
    let ico_data: &'static [u8] = include_bytes!("../webpages/favicon.ico");
    let content_type = ContentType::Icon;

    (content_type, ico_data)
}


/// The main function of the website.
#[launch]
async fn rocket() -> _ {
    rocket::build()
        .configure(Config {
            // pas de output
            log_level: rocket::config::LogLevel::Critical,
            port: 80,
            ..Default::default()
        })
        .mount("/", raw_routes())
        .mount("/", routes![favicon])
}