//! This module contain programs about Articles



use rocket::{form::Form, fs::TempFile, post, serde::uuid::Uuid, FromForm};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

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
    title: String, // the title of the article
    intro: String, // a short description of the article
}

// a fonction that get new articles  
#[post("/new-article", format = "multipart/form-data", data = "<form>")]
pub async fn new_article(mut form: Form<Upload<'_>>) -> std::io::Result<()> {
    
    // upload the file
    let file_id: String = Uuid::new_v4()
        .hyphenated()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_owned();
    let file_name = String::from("./data/articles/") + &file_id + ".md";
    form.file.persist_to(file_name).await?;

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
    let json_content = serde_json::to_string_pretty(&*articles);
    println!("{:?}", json_content);
    match serde_json::to_writer_pretty(&File::create("data/articles.json")?, &*articles) {
        Ok(_) => println!("Successfully wrote articles.json"),
        Err(e) => {
            println!("Failed to write articles.json: {}", e);
        }
    };

    Ok(())
}

// a fonction to load article.json a put the data into ARTICLES
pub fn load_article() {
    let file = File::open("data/articles.json").expect("Failed to open articles.json");
    // on affichie si il y a une erreur
    let articles: HashMap<Uuid, Article> =
        serde_json::from_reader(file).expect("Failed to read articles");
    super::ARTICLES
        .write()
        .unwrap()
        .extend(articles);
}
