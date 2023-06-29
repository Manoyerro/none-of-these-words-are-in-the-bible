use crate::models::wordinfo::WordInfo;
use std::collections::HashMap;

pub fn get_file_as_map() -> HashMap<String, Vec<WordInfo>> {
    let words_string = std::fs::read_to_string("assets/words.json").expect("Words file not found");
    serde_json::from_str(&words_string).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::models::wordinfo::WordInfo;
    use crate::parser::get_file_as_map;

    #[test]
    fn get_file_as_map_returns_file() {
        let map = get_file_as_map();
        assert_eq!(map.len(), 13399)
    }

    #[test]
    fn get_file_as_map_returns_accurate_word_info() {
        let map = get_file_as_map();
        let expected = &vec![WordInfo {
            book: "Joel".to_string(),
            chapter: 3,
            verse: 3,
        }];
        let actual = map.get("girl").unwrap();
        assert_eq!(expected, actual)
    }
}
