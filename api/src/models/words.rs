use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Words {
    pub words: String,
}

impl Words {
    pub fn from_text(text: String) -> Self {
        Words { words: text }
    }
}
