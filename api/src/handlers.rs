use crate::models::matchinfo::MatchInfo;
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
    let found_word_info = BIBLE_WORDS.get(word)?;
    Some(ReturnInfo {
        start_pos: index,
        end_pos: index + word.len() - 1,
        matches: found_word_info
            .into_iter()
            .map(|item| MatchInfo {
                word_info: item,
                link: format!(
                    "https://www.kingjamesbibleonline.org/{}-{}-{}/",
                    item.book, item.chapter, item.verse
                ),
            })
            .collect(),
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("/lookup", web::post().to(get_frequency)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    use actix_web::{
        body::to_bytes,
        http::{self},
    };
    use assert_json_diff::assert_json_eq;
    use serde_json::{json, Value};
    use std::str::from_utf8;

    static GIRL_WORD_INFO: Lazy<WordInfo> = Lazy::new(|| WordInfo {
        book: "Joel".to_string(),
        chapter: 3,
        verse: 3,
    });

    static GIRL_MATCH_INFO: Lazy<MatchInfo> = Lazy::new(|| MatchInfo {
        word_info: &GIRL_WORD_INFO,
        link: "https://www.kingjamesbibleonline.org/Joel-3-3/".to_string(),
    });

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
        let expected_output: Vec<ReturnInfo> = vec![ReturnInfo {
            start_pos: 0,
            end_pos: 3,
            matches: vec![GIRL_MATCH_INFO.clone()],
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
        let expected_output: Vec<ReturnInfo> = vec![
            ReturnInfo {
                start_pos: 0,
                end_pos: 3,
                matches: vec![GIRL_MATCH_INFO.clone()],
            },
            ReturnInfo {
                start_pos: 5,
                end_pos: 8,
                matches: vec![GIRL_MATCH_INFO.clone()],
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
        let expected_output: Vec<ReturnInfo> = vec![ReturnInfo {
            start_pos: 0,
            end_pos: 3,
            matches: vec![GIRL_MATCH_INFO.clone()],
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
        let expected_output: Vec<ReturnInfo> = vec![ReturnInfo {
            start_pos: 0,
            end_pos: 3,
            matches: vec![GIRL_MATCH_INFO.clone()],
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
            matches: vec![GIRL_MATCH_INFO.clone()],
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
