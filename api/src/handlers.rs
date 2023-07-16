use crate::models::returninfo::ReturnInfo;
use crate::models::wordinfo::WordInfo;
use crate::models::words::Words;
use crate::parser::get_file_as_map;
use actix_web::web;
use actix_web::{web::Json, HttpResponse};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

static BIBLE_WORDS: Lazy<HashMap<String, Vec<WordInfo>>> = Lazy::new(|| get_file_as_map());

pub async fn get_frequency(words: Json<Words>) -> HttpResponse {
    let split_words = words.words.unicode_word_indices();
    let word_freq: Vec<ReturnInfo> = split_words
        .map(|(index, word)| lookup(index, &word.to_lowercase()))
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .collect();
    HttpResponse::Ok().json(word_freq)
}

fn lookup<'a>(index: usize, word: &str) -> Option<ReturnInfo<'a>> {
    match BIBLE_WORDS.get(word) {
        Some(found_word_info) => Some(ReturnInfo {
            start_pos: index,
            end_pos: index + word.len() - 1,
            matches: Vec::from_iter(found_word_info.iter()),
            links: found_word_info
                .iter()
                .map(|item| format!("https://www.kingjamesbibleonline.org/{}-{}-{}/", item.book, item.chapter, item.verse))
                .collect()
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
    use assert_json_diff::assert_json_eq;
    use serde_json::{json, Value};
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
        let actual = response_to_json_value(resp).await;
        assert_json_eq!(actual, json!(expected_output))
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
            start_pos: 0,
            end_pos: 3,
            matches: vec![&expected_info],
            links: Vec::new()
        }];
        let json_words = Json(test_words);
        let resp = get_frequency(json_words).await;
        let actual = response_to_json_value(resp).await;
        assert_json_eq!(actual, json!(expected_output))
    }

    #[actix_web::test]
    async fn test_repeated_present_word_returns_array_of_return_info() {
        let test_words = Words {
            words: "Girl Girl".to_string(),
        };
        let expected_info = WordInfo {
            book: "Joel".to_string(),
            chapter: 3,
            verse: 3,
        };
        let expected_output: Vec<ReturnInfo> = vec![
            ReturnInfo {
                start_pos: 0,
                end_pos: 3,
                matches: vec![&expected_info],
                links: Vec::new()
            },
            ReturnInfo {
                start_pos: 5,
                end_pos: 8,
                matches: vec![&expected_info],
                links: Vec::new()
            },
        ];
        let json_words = Json(test_words);
        let resp = get_frequency(json_words).await;
        let actual = response_to_json_value(resp).await;
        assert_json_eq!(actual, json!(expected_output))
    }

    #[actix_web::test]
    async fn test_present_and_missing_word_returns_array_of_return_info() {
        let test_words = Words {
            words: "Girl Fauna".to_string(),
        };
        let expected_info = WordInfo {
            book: "Joel".to_string(),
            chapter: 3,
            verse: 3,
        };
        let expected_output: Vec<ReturnInfo> = vec![ReturnInfo {
            start_pos: 0,
            end_pos: 3,
            matches: vec![&expected_info],
            links: Vec::new()
        }];
        let json_words = Json(test_words);
        let resp = get_frequency(json_words).await;
        let actual = response_to_json_value(resp).await;
        assert_json_eq!(actual, json!(expected_output))
    }

    #[actix_web::test]
    async fn test_present_lowercase_returns_array_of_return_info() {
        let test_words = Words {
            words: "girl".to_string(),
        };
        let expected_info = WordInfo {
            book: "Joel".to_string(),
            chapter: 3,
            verse: 3,
        };
        let expected_output: Vec<ReturnInfo> = vec![ReturnInfo {
            start_pos: 0,
            end_pos: 3,
            matches: vec![&expected_info],
            links: Vec::new()
        }];
        let json_words = Json(test_words);
        let resp = get_frequency(json_words).await;
        let actual = response_to_json_value(resp).await;
        assert_json_eq!(actual, json!(expected_output))
    }

    #[actix_web::test]
    async fn test_present_mixed_case_returns_array_of_return_info() {
        let test_words = Words {
            words: "gIrL".to_string(),
        };
        let expected_info = WordInfo {
            book: "Joel".to_string(),
            chapter: 3,
            verse: 3,
        };
        let expected_output: Vec<ReturnInfo> = vec![ReturnInfo {
            start_pos: 0,
            end_pos: 3,
            matches: vec![&expected_info],
            links: Vec::new()
        }];
        let json_words = Json(test_words);
        let resp = get_frequency(json_words).await;
        let actual = response_to_json_value(resp).await;
        assert_json_eq!(actual, json!(expected_output))
    }

    async fn response_to_json_value(resp: HttpResponse) -> Value {
        let byte_body = to_bytes(resp.into_body()).await.unwrap();
        let string_body = from_utf8(&byte_body).unwrap();
        serde_json::from_str(string_body).unwrap()
    }
}
