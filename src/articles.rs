//! This module contain programs about Articles

use rand::Rng;
use rocket::http::CookieJar;
use rocket::response::Redirect;
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
pub async fn new_article(mut form: Form<Upload<'_>>, cookies: &CookieJar<'_>) -> Redirect {
    // check if the user is admin
    if !super::is_admin(cookies) {
        return Redirect::to("/");
    }

    // upload the file
    // gen a random id
    let random_uuid = Uuid::new_v4();

    let file_id = random_uuid
        .hyphenated()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_owned();

    // create the name of the file
    let file_name = String::from("./data/articles/") + &file_id + ".md";
    // copy the file into data
    if let Err(e) = form.file.copy_to(file_name).await {
        println!("error during copy of the markdown file: {}", e);
        return Redirect::to("/admin");
    }

    let img_name = String::from("./data/img/") + &file_id + ".png";
    if let Err(e) = form.img.copy_to(img_name).await {
        println!("error during copy of the image file: {}", e);
        return Redirect::to("/admin");
    }

    // create an article.
    let article = Article {
        title: form.title.clone(),
        intro: form.intro.clone(),
        id: file_id.clone(),
    };

    // save the article
    let articles = super::ARTICLES.write();
    if let Ok(mut articles) = articles {
        articles.insert(random_uuid, article);

        // get file in hard memory
        let memory_file = File::create("data/articles.json");
        if let Ok(memory_file) = memory_file {
            // write in memory
            if let Err(e) = serde_json::to_writer_pretty(memory_file, &*articles) {
                println!("Failed to write articles.json: {}", e);
            };
        } else {
            println!("Failed to create articles.json : {:?}", memory_file);
        }
    } else {
        println!("Failed to write articles.json : {:?}", articles);
    }

    // redirect to the admin page
    Redirect::to("/admin")
}

// a fonction to load article.json a put the data into ARTICLES
pub fn load_article() {
    // open the file
    let file = File::open("data/articles.json");
    if let Ok(file) = file {
        // read the file and put the data into ARTICLES if it didn't work then create an empty hashmap
        let articles: HashMap<Uuid, Article> =
            serde_json::from_reader(file).unwrap_or_else(|_| HashMap::new());

        // write the data into ARTICLES
        if let Ok(mut all_articles) = super::ARTICLES.write() {
            all_articles.extend(articles);
        } else {
            println!("Failed to write articles.json into : {:?}", super::ARTICLES);
        }
    } else {
        println!("Failed to open articles.json : {:?}", file);
    }
}

// a fonction to delete article
#[post(
    "/delete_article",
    data = "<id>",
    format = "application/x-www-form-urlencoded"
)]
pub fn delete_article(id: Form<Uuid>, cookies: &CookieJar<'_>) -> Redirect {
    // check if the user is admin
    if !super::is_admin(cookies) {
        return Redirect::to("/");
    }

    let articles = super::ARTICLES.write();
    if let Ok(mut articles) = articles {
        articles.remove(&id);

        if let Err(e) = std::fs::remove_file(format!("data/articles/{}.md", id.to_string())) {
            println!("Failed to delete markdown file : {}", e);
        }
        if let Err(e) = std::fs::remove_file(format!("data/img/{}.png", id.to_string())) {
            println!("Failed to delete image file : {}", e);
        }

        // get file in hard memory
        let memory_file = File::create("data/articles.json");
        if let Ok(memory_file) = memory_file {
            // write in memory
            serde_json::to_writer_pretty(memory_file, &*articles)
                .unwrap_or_else(|_| println!("Failed to write articles.json"));
        } else {
            println!("Failed to get or create articles.json : {:?}", memory_file);
        }
        return Redirect::to("/admin");
    }
    Redirect::to("/admin")
}

/// a function to get a random article
#[get("/random")]
pub fn get_random_article() -> Json<Option<Article>> {
    // manage if articles are empty
    let articles = super::ARTICLES.read();

    if let Err(e) = articles {
        println!("Failed to read articles.json : {:?}", e);
        return Json(None);
    } else if let Ok(articles) = articles {
        if articles.is_empty() {
            return Json(None);
        }

        // get all keys
        let keys: Vec<&Uuid> = articles.keys().collect();
        // get a random index for random keys
        let random_index = rand::thread_rng().gen_range(0..keys.len());

        if let Some(article) = articles.get(keys[random_index]) {
            // return the article at the random index
            return Json(Some(article.clone()));
        } else {
            return Json(None);
        }
    } else {
        Json(None)
    }
}

/// a function to get all articles in  a list
#[get("/list")]
pub fn get_article_list() -> Json<Vec<Uuid>> {
    // get all articles
    let articles = super::ARTICLES.read();
    if let Ok(articles) = articles {
        // return the list of all articles
        return Json(articles.keys().copied().collect());
    } else {
        println!("Failed to read articles.json : {:?}", articles);
        Json(vec![])
    }
}

// a function to get an article with its id.
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
    let article = super::ARTICLES.read();

    if let Ok(articles) = article {
        if let Some(article) = articles.get(&id) {
            return Some(Json(article.clone()));
        }
    }
    None
}

/// we recup the content of the file and return it
#[get("/get/<id>")]
pub async fn get_article(id: Uuid) -> String {
    let file = File::open(format!("./data/articles/{}.md", id));
    if let Ok(mut file) = file {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap_or_else(|_| {
            println!("Failed to read file");
            0
        });

        return contents;
    }
    println!("Failed to open file");
    "Failed to read file".to_string()
}
