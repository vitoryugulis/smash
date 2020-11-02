use chrono::{NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub nickname: String,
    pub email: String,
    pub telephone: String,
    pub birth_date: NaiveDate,
    pub address: String,
    pub create_date: NaiveDateTime,
    pub update_date: Option<NaiveDateTime>,
    pub status_id: i64,
}
