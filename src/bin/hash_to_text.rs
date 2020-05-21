use rust_sodiumoxide_examples::hashing;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(passwd) => {
            println!("hashing {}", passwd);
            let (texthash, _) = hashing::hash(passwd);
            println!(
                "texthash trimmed len = {}",
                texthash.trim_end_matches('\u{0}').len()
            );
            println!("texthash = '{}'", texthash.trim_end_matches('\u{0}'));
        }
        _ => {
            println!("nothing to do");
        }
    }
}
