use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::{get, http::ContentType, post};
use sha1::{Digest, Sha1};

/// a function to login as admin.
#[post("/login", data = "<password>")]
pub async fn login_admin(cookies: &CookieJar<'_>, password: String) -> Redirect {
    if sha1_hash(&password) == "0e8388d85abbb460763881d334ba6593f1199977" {
        // on ajoute le cookie prive
        cookies.add_private(Cookie::new("admin", "true"));
    }
    Redirect::to("/admin")
}

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

/// a function to hash a string.
fn sha1_hash(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}

/// a fonction to get if the user is admin.
pub fn is_admin(cookies: &CookieJar<'_>) -> bool {
    cookies
        .get_private("admin")
        .map_or(false, |cookie| cookie.value() == "true")
}
