use crate::models::returninfo::ReturnInfo;
use crate::models::wordinfo::WordInfo;
use crate::models::words::Words;
use crate::parser::get_file_as_map;
use actix_web::web;
use actix_web::{web::Json, HttpResponse};
use std::collections::HashMap;

pub async fn get_frequency(words: Json<Words>) -> HttpResponse {
    // TODO: See if we can move this to a static location
    let bible_words = get_file_as_map();
    let split_words = words.words.split(" ");
    let word_freq: Vec<Option<ReturnInfo>> =
        split_words.map(|word| lookup(&bible_words, word)).collect();

    HttpResponse::Ok().json(word_freq)
}

fn lookup<'a>(
    bible_words: &'a HashMap<String, Vec<WordInfo>>,
    word: &str,
) -> Option<ReturnInfo<'a>> {
    let found_word_info = bible_words.get(word);
    match found_word_info {
        Some(info) => Some(ReturnInfo {
            matches: Vec::from_iter(info.iter()),
        }),
        None => None,
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("/test", web::post().to(get_frequency)));
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{
        body::to_bytes,
        http::{self},
    };
    use serde_json::json;
    use std::str::from_utf8;

    #[actix_web::test]
    async fn test_response_ok() {
        let test_words = Words {
            words: "I miss Fauna".to_string(),
        };
        let json_words = Json(test_words);
        let resp = get_frequency(json_words).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_body_ok() {
        let test_words = Words {
            words: "I miss Fauna".to_string(),
        };
        let expected_output: Vec<WordInfo> = vec![
            WordInfo {
                book: "I".to_string(),
                chapter: 1,
                verse: 1,
            },
            WordInfo {
                book: "miss".to_string(),
                chapter: 1,
                verse: 1,
            },
            WordInfo {
                book: "Fauna".to_string(),
                chapter: 1,
                verse: 1,
            },
        ];
        let json_words = Json(test_words);
        let resp = get_frequency(json_words).await;
        let byte_body = to_bytes(resp.into_body()).await.unwrap();
        let string_body = from_utf8(&byte_body).unwrap();
        assert_eq!(string_body, json!(expected_output).to_string())
    }
}
