use crate::models::wordinfo::WordInfo;
use serde::Serialize;

#[derive(Serialize)]
pub struct ReturnInfo<'a> {
    pub matches: Vec<&'a WordInfo>,
    // TODO: Expand with other fields
}
