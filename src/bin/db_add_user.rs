use rust_sodiumoxide_examples::database;
use rust_sodiumoxide_examples::hashing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(2) {
        Some(passwd) => {
            //
            // hash the password and add the user to the db
            //
            let user_name = args.get(1).unwrap();
            let (texthash, hashed_password) = hashing::hash(passwd);
            let result = database::add_user(database::UserDBRecord {
                user_name: String::from(user_name),
                password_hash_char: texthash.trim_end_matches('\u{0}').to_string(),
                password_hash_bin: hashed_password,
                email_address: String::from("email@email.test"),
            })
            .await?;

            match result {
                1 => {
                    println!("added user {}", user_name);
                    //
                    // fetch the user info from the db and test password verification
                    //
                    let user = database::get_user(String::from(args.get(1).unwrap())).await?;
                    let mut padded = [0u8; 128];
                    padded[..user.password_hash_char.len()].copy_from_slice(user.password_hash_char.as_bytes());
                    println!(
                        "verify with password_hash_char: {:?}",
                        hashing::verify(padded, passwd)
                    );
                    let mut binarray = [0u8;128];
                    binarray.copy_from_slice(&user.password_hash_bin[..]);
                    println!(
                        "verify with password_hash_bin: {:?}",
                        hashing::verify(binarray, passwd)
                    );
                }
                _ => println!("failed to add user {}", user_name),
            }
        }
        _ => {
            println!("nothing to do");
        }
    }
    Ok(())
}
