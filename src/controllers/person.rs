
use chrono::NaiveDate;
use chrono::{Local};
use mysql::{Pool};
use crate::database::{objects::{login::Login, person::Person}, person};
use crate::models;
use crate::models::generic_error::GenericError;

pub async fn create(
    data: models::person::Person, 
    pool: &Pool,
) -> Result<String, GenericError> {
    let now = Local::now();
    let birth_date = NaiveDate::parse_from_str(data.birth_date.as_str(), "%Y-%m-%d");
    let b  = match birth_date {
        Ok(b) => b,
        Err(e) => return Err(GenericError{error: format!("Formato errado da data: {}", e.to_string())})
    };

    let person = Person{
        id: 0,
        name: data.name,
        nickname: data.nickname.clone(),
        email: data.email,
        telephone: data.telephone,
        birth_date: b,
        address: data.address,
        create_date: now.clone().naive_local(),
        update_date: None,
        status_id: 1,
    };

    let person_id = match person::create_user(person, pool) {
        Ok(r) => r,
        Err(e) => return Err(GenericError{error: e.to_string()})
    };

    let login = Login {
        id : 0,
        user_login: data.nickname,
        user_password: data.password,
        create_date: now.clone().naive_local(),
        facebook_token: None,
        instagram_token: None,
        google_token: None,
        person_id: person_id,
    };

    match person::create_login(login, pool) {
        Ok(r) => r,
        Err(e) => return Err(GenericError{error: e.to_string()})
    };
    
    Ok("token".to_string())
}