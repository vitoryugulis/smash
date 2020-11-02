use mysql::*;
use mysql::prelude::*;

use super::objects::{login::Login, person::Person};

pub fn create_user(
    data: Person, 
    pool: &Pool,
) -> Result<u64> {
    let mut conn = pool.get_conn()?;
    let stmt = "
    INSERT INTO Person (
        name,
        nickname,
        email,
        telephone,
        birthDate,
        address,
        createDate,
        statusId
    ) VALUES
    (
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?
    )
    ";
    let mut params = Vec::<String>::new();
    params.push(data.name);
    params.push(data.nickname);
    params.push(data.email);
    params.push(data.telephone);
    params.push(data.birth_date.to_string());
    params.push(data.address);
    params.push(data.create_date.to_string());
    params.push(data.status_id.to_string());
    conn.exec_drop(stmt, params)?;
    let id = conn.last_insert_id();

    return Ok(id);
}

pub fn create_login(
    data: Login, 
    pool: &Pool,
) -> Result<u64> {
    let mut conn = pool.get_conn()?;
    let stmt = "
    INSERT INTO UserLogin (
        userLogin,
        userPassword,
        createDate,
        personId
    ) VALUES
    (
        ?,
        ?,
        ?,
        ?
    )
    ";
    let mut params = Vec::<String>::new();
    params.push(data.user_login);
    params.push(data.user_password);
    params.push(data.create_date.to_string());
    params.push(data.person_id.to_string());
    conn.exec_drop(stmt, params)?;
    let id = conn.last_insert_id();

    return Ok(id);
}