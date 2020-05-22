use rust_sodiumoxide_examples::hashing;

fn main() {
    let passwd = "bob is cool";

    //
    // bob_is_cool_hash does not contain 128 chars
    // bob_is_cool_hash only has 97 chars
    // bob_is_cool_hash needs padded with NULLs to 128 chars or verification will fail
    //
    let bob_is_cool_hash = "$argon2id$v=19$m=65536,t=2,p=1$SeK5rTtVL6NZx6fUledorg$6uRohRxwfSmVhSVa4gHScK/5Yv5jQ5gJ7Bw1/H89BgA";

    //
    // this doesn't work bc the slice is missing the \u{0} padding
    //
    println!(
        "verifying without padded hash (len = {}): {:?}",
        bob_is_cool_hash.len(),
        passwd
    );
    println!(
        "{}",
        hashing::verify_slice(bob_is_cool_hash.as_bytes(), passwd)
    );

    //
    // create a padded array size 128
    //
    let mut padded = [0u8; 128];
    let hash = bob_is_cool_hash.as_bytes();
    padded[..hash.len()].copy_from_slice(hash);
    println!(
        "verifying with padded hash (len = {}): {:?}",
        padded.len(),
        passwd
    );
    println!("{}", hashing::verify_slice(&padded, passwd));
}
