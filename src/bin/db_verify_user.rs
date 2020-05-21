use rust_sodiumoxide_examples::database;
use rust_sodiumoxide_examples::hashing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(2) {
        Some(passwd) => {
            //
            // hash the given password and add compare to
            // the hash stored in the database
            //
            let user = database::get_user(String::from(args.get(1).unwrap())).await?;
            let mut padded = [0u8; 128];
            user.password_hash_char
                .as_bytes()
                .iter()
                .enumerate()
                .for_each(|(i, val)| {
                    padded[i] = val.clone();
                });
            println!(
                "verify with password_hash_char: {:?}",
                hashing::verify(padded, passwd)
            );
            println!(
                "verify with password_hash_bin: {:?}",
                hashing::verify(user.password_hash_bin.0, passwd)
            );
        }
        _ => {
            println!("nothing to do");
        }
    }

    Ok(())
}
