use serde::{Serialize, Deserialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub comment: String,
    pub post_id: i64,
    pub person_id: i64,
    pub status_id: i64,
}
