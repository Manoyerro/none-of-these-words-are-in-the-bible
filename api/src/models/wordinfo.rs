use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct WordInfo {
    pub book: String,
    pub chapter: u8,
    pub verse: u8,
}
