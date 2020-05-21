use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::pwhash::argon2id13::HashedPassword;
use sqlx::{Connect, FromRow, PgConnection};
use std::env;

// user data from db (includes id)
#[derive(Debug, Serialize, FromRow)]
pub struct UserDBRecordWithId {
    pub id: i32,
    pub user_name: String,
    pub password_hash_char: String,
    pub password_hash_bin: HashedPassword,
    pub email_address: String,
}

// user data to db (no id)
#[derive(Deserialize, FromRow)]
pub struct UserDBRecord {
    pub user_name: String,
    pub password_hash_char: String,
    pub password_hash_bin: HashedPassword,
    pub email_address: String,
}

pub async fn add_user(user: UserDBRecord) -> Result<u64, sqlx::Error> {
    let conn =
        PgConnection::connect(&env::var("DATABASE_URL").expect("can't get no env::var")).await?;

    let result = sqlx::query!(
        r#"INSERT INTO users(user_name,password_hash_bin,password_hash_char,email_address)
            VALUES ($1,$2,$3,$4)
            ON CONFLICT(user_name) DO UPDATE SET password_hash_bin = $2
        "#,
        user.user_name,
        &user.password_hash_bin.0[..],
        user.password_hash_char,
        user.email_address
    )
    .execute(conn)
    .await;

    result
}

pub async fn get_user(user_name: String) -> Result<UserDBRecordWithId, sqlx::Error> {
    let mut conn =
        PgConnection::connect(&env::var("DATABASE_URL").expect("can't get no env::var")).await?;

    let user_select = sqlx::query!(
        r#"
        SELECT id,user_name,password_hash_bin,password_hash_char,email_address 
        FROM users WHERE user_name = $1"#,
        user_name
    )
    .fetch_one(&mut conn)
    .await?;

    Ok(UserDBRecordWithId {
        id: user_select.id,
        user_name: user_select.user_name,
        password_hash_bin: HashedPassword::from_slice(&user_select.password_hash_bin).unwrap(),
        password_hash_char: user_select.password_hash_char,
        email_address: user_select.email_address,
    })
}
