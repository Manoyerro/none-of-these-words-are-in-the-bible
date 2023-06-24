use actix_web::web;
use actix_web::{web::{
    Json,
}, post, HttpResponse};
use crate::parser::WordInfo;
use crate::{models::words::Words};


#[post("/test")]
pub async fn get_frequency(words: Json<Words>) -> HttpResponse {
    let mut word_freq : Vec<WordInfo> = Vec::new();  
    let split_words: Vec<&str> = words.words.split(" ").collect();
    for word in split_words{
        // Compute the frequency and details and such
    }
    HttpResponse::Ok().json(word_freq)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_frequency)
    );
}