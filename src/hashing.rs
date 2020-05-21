use sodiumoxide::crypto::pwhash::argon2id13;
use std::time::Instant;

pub fn hash(passwd: &str) -> (String, argon2id13::HashedPassword) {
    sodiumoxide::init().unwrap();
    let hash = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();
    let texthash = std::str::from_utf8(&hash.0).unwrap().to_string();
    (texthash, hash)
}

pub fn hash_timed() {
    let passwd = "Hashy McHasherton";

    // perform hashing without calling init() first.
    // like strolling through a park
    let now = Instant::now();
    let _ = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();
    println!(
        "INTERACTIVE hashing without init() took {} milliseconds.",
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    let _ = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_MODERATE,
        argon2id13::MEMLIMIT_MODERATE,
    )
    .unwrap();
    println!(
        "MODERATE hashing without init() took {} milliseconds.",
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    let _ = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_SENSITIVE,
        argon2id13::MEMLIMIT_SENSITIVE,
    )
    .unwrap();
    println!(
        "SENSITIVE hashing without init() took {} milliseconds.",
        now.elapsed().as_millis()
    );

    // initialize...
    println!("\n### sodiumoxide::init() ###");
    let now = Instant::now();
    sodiumoxide::init().unwrap();
    println!("init() took {} milliseconds.\n", now.elapsed().as_millis());

    // hash again after calling init()
    // buckle up!!
    let now = Instant::now();
    let _ = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();
    println!(
        "INTERACTIVE hashing after init() took {} milliseconds.",
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    let _ = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_MODERATE,
        argon2id13::MEMLIMIT_MODERATE,
    )
    .unwrap();
    println!(
        "MODERATE hashing after init() took {} milliseconds.",
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    let _ = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_SENSITIVE,
        argon2id13::MEMLIMIT_SENSITIVE,
    )
    .unwrap();
    println!(
        "SENSITIVE hashing after init() took {} milliseconds.",
        now.elapsed().as_millis()
    );
}

pub fn verify_slice(hash: &[u8], passwd: &str) -> bool {
    sodiumoxide::init().unwrap();
    match argon2id13::HashedPassword::from_slice(&hash) {
        Some(hp) => argon2id13::pwhash_verify(&hp, passwd.as_bytes()),
        _ => false,
    }
}

pub fn verify(hash: [u8; 128], passwd: &str) -> bool {
    sodiumoxide::init().unwrap();
    match argon2id13::HashedPassword::from_slice(&hash) {
        Some(hp) => argon2id13::pwhash_verify(&hp, passwd.as_bytes()),
        _ => false,
    }
}
