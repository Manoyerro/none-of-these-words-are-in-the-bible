use crate::models::wordinfo::WordInfo;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct MatchInfo<'a> {
    #[serde(flatten)]
    pub word_info: &'a WordInfo,
    pub link: String,
}
