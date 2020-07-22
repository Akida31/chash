use crate::hashing::get_algorithm_by_hash_len;
use std::io::Write;
use std::path::PathBuf;

mod hashing;

fn get_path() -> PathBuf {
    let mut input = String::new();
    loop {
        print!("Please put in the path: ");
        std::io::stdout().flush().expect("Output error");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Can't read input");
        input = input.trim().parse().unwrap();
        let path = PathBuf::from(&input);
        if path.is_file() || path.is_dir() {
            break path;
        } else {
            println!("path {:?} not found", path);
        }
    }
}

fn get_algorithm() -> (String, String) {
    let mut input = String::new();
    print!("Please put in the hash or leave it empty to compute the hash only: ");
    std::io::stdout().flush().expect("Output error");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can't read input");
    let given_hash: String = input.trim().parse().unwrap();
    match get_algorithm_by_hash_len(given_hash.len()) {
        Some(a) => (a, given_hash),
        None => loop {
            print!("Please put in the algorithm: ");
            std::io::stdout().flush().expect("Output error");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Can't read input");
            let algorithm: String = input.trim().parse().unwrap();
            if hashing::available_algorithms().contains(&algorithm) {
                break (algorithm, given_hash);
            } else {
                println!("Algorithm not available\nAvailable algorithms:");
                for algo in hashing::available_algorithms() {
                    println!("  {}", algo);
                }
            }
        },
    }
}

fn main() {
    // TODO cli with args
    let file = get_path();
    let (algorithm, given_hash) = get_algorithm();
    let computed_hash = hashing::hash(file, &*algorithm).unwrap();
    if given_hash.len() < 2 {
        println!("{}", computed_hash);
    } else if computed_hash == given_hash {
        println!("The hashes are equal");
    } else {
        println!("ERROR\nThe hashes are NOT equal");
    }
}
