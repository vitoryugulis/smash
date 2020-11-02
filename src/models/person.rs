use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub name: String,
    pub nickname: String,
    pub email: String,
    pub telephone: String,
    pub birth_date: String,
    pub address: String,
    pub password: String,
}
