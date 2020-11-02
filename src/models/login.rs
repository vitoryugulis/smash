use serde::{Serialize, Deserialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    pub login: String,
    pub password: String,
}
