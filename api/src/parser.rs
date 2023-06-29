use crate::models::wordinfo::WordInfo;
use std::collections::HashMap;

pub fn get_file_as_map() -> HashMap<String, Vec<WordInfo>> {
    let words_string = std::fs::read_to_string("assets/words.json").expect("Words file not found");
    serde_json::from_str(&words_string).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::parser::get_file_as_map;

    #[test]
    fn get_file_as_map_returns_file() {
        let map = get_file_as_map();
        assert_eq!(map.len(), 13399)
    }
}
