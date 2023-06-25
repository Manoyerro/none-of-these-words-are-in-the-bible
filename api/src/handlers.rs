use actix_web::web;
use actix_web::{web::{
    Json,
}, post, HttpResponse};
use crate::parser::WordInfo;
use crate::{models::words::Words};

pub async fn get_frequency(words: Json<Words>) -> HttpResponse {
    let mut word_freq : Vec<WordInfo> = Vec::new();  
    let split_words: Vec<&str> = words.words.split(" ").collect();
    for word in split_words{
        // Compute the frequency and details and such
        word_freq.push(WordInfo { book: word.to_string(), chapter: 1, verse: 1 })
    }
    HttpResponse::Ok().json(word_freq)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/test", web::post().to(get_frequency))
    );
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{
        http::{self},
        body::{to_bytes},
    };
    use serde_json::json;
    use std::str::from_utf8;

    #[actix_web::test]
    async fn test_response_ok() {
        let test_words = Words{
            words: "I miss Fauna".to_string(),
        };
        let json_words = web::Json(test_words);
        let resp = get_frequency(json_words).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_body_ok() {
        let test_words = Words{
            words: "I miss Fauna".to_string(),
        };
        let expected_output: Vec<WordInfo> = vec![
            WordInfo{ book: "I".to_string(), chapter: 1, verse: 1 },
            WordInfo{ book: "miss".to_string(), chapter: 1, verse: 1 },
            WordInfo{ book: "Fauna".to_string(), chapter: 1, verse: 1 },

        ];
        let json_words = web::Json(test_words);
        let resp = get_frequency(json_words).await;
        let byte_body = to_bytes(resp.into_body()).await.unwrap();
        let string_body = from_utf8(&byte_body).unwrap();
        assert_eq!(string_body, json!(expected_output).to_string())
    }
}