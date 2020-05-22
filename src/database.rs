// use sodiumoxide::crypto::pwhash::argon2id13::HashedPassword;
use sqlx::{Connect, FromRow, PgConnection};
use uuid::Uuid;

// user data from db (includes id)
#[derive(FromRow)]
pub struct UserDBRecordWithId {
    pub id: Uuid,
    pub user_name: String,
    pub password_hash_char: String,
    pub password_hash_bin: Vec<u8>,
    pub email_address: String,
}

// user data to db (no id)
#[derive(FromRow)]
pub struct UserDBRecord {
    pub user_name: String,
    pub password_hash_char: String,
    pub password_hash_bin: Vec<u8>,
    pub email_address: String,
}

pub async fn add_user(user: UserDBRecord) -> Result<u64, sqlx::Error> {
    let conn =
        PgConnection::connect(&std::env::var("DATABASE_URL").expect("can't get no env::var"))
            .await?;

    let result = sqlx::query!(
        r#"INSERT INTO sodium.users(user_name,password_hash_bin,password_hash_char,email_address)
            VALUES ($1,$2,$3,$4)
            ON CONFLICT(user_name) DO UPDATE SET password_hash_bin = $2
        "#,
        user.user_name,
        user.password_hash_bin,
        user.password_hash_char,
        user.email_address
    )
    .execute(conn)
    .await;

    result
}

pub async fn get_user(user_name: String) -> Result<UserDBRecordWithId, sqlx::Error> {
    let mut conn =
        PgConnection::connect(&std::env::var("DATABASE_URL").expect("can't get no env::var"))
            .await?;

    Ok(sqlx::query_as!(
        UserDBRecordWithId,
        r#"
        SELECT id,user_name,password_hash_bin,password_hash_char,email_address 
        FROM sodium.users WHERE user_name = $1"#,
        user_name
    )
    .fetch_one(&mut conn)
    .await?)
}
