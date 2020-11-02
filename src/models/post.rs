use serde::{Serialize, Deserialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub title: String,
    pub content: String,
}
