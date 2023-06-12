use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct WordInfo {
    book: String,
    chapter: u8,
    verse: u8,
}

pub async fn get_file_as_map() -> HashMap<String, Vec<WordInfo>> {
    let words_string = std::fs::read_to_string("assets/words.json").unwrap();
    serde_json::from_str(&words_string).unwrap()
}
