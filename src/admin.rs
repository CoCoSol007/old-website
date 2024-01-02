use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::{get, http::ContentType, post};
use sha2::{Digest, Sha256};
use std::env;
use base64::{Engine as _, engine::general_purpose};

/// a function to login as admin.
#[post("/login", data = "<password>")]
pub async fn login_admin(cookies: &CookieJar<'_>, password: String) -> Redirect {
    // get the env variable password
    let env_password = env::var("ADMIN_PASSWORD").unwrap_or_default();
    let env_password = general_purpose::STANDARD_NO_PAD.decode(env_password).expect("env password invalid");
    if sha1_hash(&password).as_slice()
        == env_password
    {
        // on ajoute le cookie prive
        cookies.add_private(Cookie::new("admin", "true"));
    }
    Redirect::to("/admin")
}

/// a function for send the admin main page.
#[get("/")]
pub fn admin_main(cookies: &CookieJar<'_>) -> (ContentType, &'static str) {
    if is_admin(cookies) {
        (
            ContentType::HTML,
            include_str!("../webpages/admin/main.html"),
        )
    } else {
        (
            ContentType::HTML,
            include_str!("../webpages/admin/login.html"),
        )
    }
}

/// a function for send the new article page.
#[get("/new_article")]
pub fn new_article_page(cookies: &CookieJar<'_>) -> (ContentType, &'static str) {
    if is_admin(cookies) {
        (
            ContentType::HTML,
            include_str!("../webpages/admin/new_article.html"),
        )
    } else {
        (
            ContentType::HTML,
            include_str!("../webpages/admin/login.html"),
        )
    }
}


/// a function for send the modify article page.
#[get("/modify_article")]
pub fn modify_article_page(cookies: &CookieJar<'_>) -> (ContentType, &'static str) {
    if is_admin(cookies) {
        (
            ContentType::HTML,
            include_str!("../webpages/admin/edit_articles.html"),
        )
    } else {
        (
            ContentType::HTML,
            include_str!("../webpages/admin/login.html"),
        )
    }
}

/// a function to hash a string.
fn sha1_hash(input: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    hasher.update("String data");
    // Note that calling `finalize()` consumes hasher
    hasher.finalize().into()
}

/// a fonction to get if the user is admin.
pub fn is_admin(cookies: &CookieJar<'_>) -> bool {
    cookies
        .get_private("admin")
        .map_or(false, |cookie| cookie.value() == "true")
}
