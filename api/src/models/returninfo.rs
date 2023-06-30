use crate::models::wordinfo::WordInfo;
use serde::Serialize;

#[derive(Serialize)]
pub struct ReturnInfo<'a> {
    pub start_pos: usize,
    pub end_pos: usize,
    pub matches: Vec<&'a WordInfo>,
}
