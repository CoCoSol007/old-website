//! This module contain programs about Articles

use rand::Rng;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::{form::Form, fs::TempFile, get, http::ContentType, post, serde::uuid::Uuid, FromForm};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{collections::HashMap, io::Read};

/// An article.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Article {
    // the title of the article
    pub title: String,
    // a short introduction
    pub intro: String,
    // the id of the article.
    pub id: String,
}

#[derive(FromForm)]
pub struct Upload<'f> {
    file: TempFile<'f>,
    img: TempFile<'f>,
    title: String, // the title of the article
    intro: String, // a short description of the article
}

// a fonction that get new articles
#[post("/new", format = "multipart/form-data", data = "<form>")]
pub async fn new_article(mut form: Form<Upload<'_>>, cookies: &CookieJar<'_>) -> std::io::Result<()> {
    if !super::is_admin(cookies) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "You don't have the permision",
        ));
    }
    // upload the file
    let file_id: String = Uuid::new_v4()
        .hyphenated()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_owned();
    let file_name = String::from("./data/articles/") + &file_id + ".md";
    form.file.persist_to(file_name).await?;

    let img_name = String::from("./data/img/") + &file_id + ".png";
    form.img.persist_to(img_name).await?;

    // create an article.
    let article = Article {
        title: form.title.clone(),
        intro: form.intro.clone(),
        id: file_id.clone(),
    };

    // save the article
    let mut articles = super::ARTICLES.write().unwrap();
    let uuid = Uuid::parse_str(&file_id).unwrap();
    articles.insert(uuid, article);

    // update the memory
    let result = serde_json::to_writer_pretty(&File::create("data/articles.json")?, &*articles);
    if result.is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to write articles",
        ));
    } else {
        return Ok(());
    }
}

// a fonction to load article.json a put the data into ARTICLES
pub fn load_article() {
    let file = File::open("data/articles.json").expect("Failed to open articles.json");
    // on affichie si il y a une erreur
    let articles: HashMap<Uuid, Article> =
        serde_json::from_reader(file).expect("Failed to read articles");
    super::ARTICLES.write().unwrap().extend(articles);
}

// a function to get a random article
#[get("/random")]
pub fn get_random_article() -> Json<Option<Article>> {
    // manage if the articles are empty
    if super::ARTICLES.read().unwrap().is_empty() {
        // we send nothing
        return Json(None);
    }
    let articles = super::ARTICLES.read().unwrap();
    let keys: Vec<&Uuid> = articles.keys().collect();
    let random_index = rand::thread_rng().gen_range(0..keys.len());
    Json(Some(articles.get(keys[random_index]).unwrap().clone()))
}

#[get("/list")]
pub fn get_article_list() -> Json<Vec<Uuid>> {
    Json(super::ARTICLES.read().unwrap().keys().copied().collect())
}

#[get("/img/<id>")]
pub fn get_images(id: Uuid) -> Option<(ContentType, Vec<u8>)> {
    let file = File::open(format!("data/img/{}.png", id.to_string()));
    if let Ok(mut file) = file {
        let mut buf = vec![];
        let opened_file = file.read_to_end(&mut buf);
        if let Ok(_) = opened_file {
            Some((ContentType::PNG, buf))
        } else {
            println!("request without great uuid !");
            None 
        }
    } else {
        println!("failed to open data/img/{}.png", id.to_owned());
        None
    }
}

/// a function to get an article with its id.
#[get("/minia/<id>")]
pub async fn get_minia_article(id: Uuid) -> Option<Json<Article>> {
    // On récupère l'accès aux articles qui sont dans un RwLock puis,
    // on récupère l'article et si il existe on le convertis en Json
    // sinon on renvoit None ce qui a pour effet de faire une erreur 404
    super::ARTICLES
        .read()
        .unwrap()
        .get(&id)
        .map(|article| Json(article.clone()))
}

#[get("/get/<id>")]
pub async fn get_article(id: Uuid) -> String {
    // on download larticle dans /data/articles/id.md
    let mut file = File::open(format!("./data/articles/{}.md", id)).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file")
        .to_string();

    contents
}
