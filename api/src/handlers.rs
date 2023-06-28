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
    let word_freq: Vec<ReturnInfo> = split_words
        .map(|word| lookup(&bible_words, word))
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .collect();
    HttpResponse::Ok().json(word_freq)
}

fn lookup<'a>(
    bible_words: &'a HashMap<String, Vec<WordInfo>>,
    word: &str,
) -> Option<ReturnInfo<'a>> {
    match bible_words.get(word) {
        Some(found_word_info) => Some(ReturnInfo {
            matches: Vec::from_iter(found_word_info.iter()),
        }),
        None => None,
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("/lookup", web::post().to(get_frequency)));
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
    async fn test_formatted_response_code_200_ok() {
        let test_words = Words {
            words: "I miss Fauna".to_string(),
        };
        let json_words = Json(test_words);
        let resp = get_frequency(json_words).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_missing_word_returns_empty_array() {
        let test_words = Words {
            words: "Fauna".to_string(),
        };
        let expected_output: Vec<ReturnInfo> = Vec::new();
        let json_words = Json(test_words);
        let resp = get_frequency(json_words).await;
        let byte_body = to_bytes(resp.into_body()).await.unwrap();
        let string_body = from_utf8(&byte_body).unwrap();
        assert_eq!(string_body, json!(expected_output).to_string())
    }

    #[actix_web::test]
    async fn test_present_word_returns_array_of_return_info() {
        let test_words = Words {
            // For some reason, this is one of the only words that only appears once in the bible/
            words: "Girl".to_string(),
        };
        let expected_info = WordInfo {
            book: "Joel".to_string(),
            chapter: 3,
            verse: 3,
        };
        let expected_output: Vec<ReturnInfo> = vec![ReturnInfo {
            matches: vec![&expected_info],
        }];
        let json_words = Json(test_words);
        let resp = get_frequency(json_words).await;
        let byte_body = to_bytes(resp.into_body()).await.unwrap();
        let string_body = from_utf8(&byte_body).unwrap();
        assert_eq!(string_body, json!(expected_output).to_string())
    }
}
