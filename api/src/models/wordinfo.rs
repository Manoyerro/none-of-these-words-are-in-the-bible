use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct WordInfo {
    pub book: String,
    pub chapter: u8,
    pub verse: u8,
}
