use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    pub id: i64,
    pub user_login: String,
    pub user_password: String,
    pub create_date: NaiveDateTime,
    pub facebook_token: Option<String>,
    pub instagram_token: Option<String>,
    pub google_token: Option<String>,
    pub person_id: u64,
}
