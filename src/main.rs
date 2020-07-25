use std::cmp::{max, min};
use std::io::Write;
use std::path::PathBuf;

mod hashing;


// TODO docs
fn string_difference(str1: String, str2: String) -> usize {
    let len1 = str1.len() + 1;
    let len2 = str2.len() + 1;
    let mut dp = Vec::with_capacity(len1);
    for i in 0..len1 {
        dp.push(Vec::with_capacity(len2));
        for j in 0..len2 {
            dp[i].push(max(i, j));
        }
    }
    for i in 1..len1 {
        for j in 1..len2 {
            let i2 = if i == 0 {len1-2} else { i -1};
            let j2 = if j == 0 {len2-2} else { j -1};
            dp[i][j] = min(
                min(
                    dp[i - 1][j - 1]
                        + if str1.chars().nth(i - 1).unwrap() != str2.chars().nth(j - 1).unwrap() {
                            1
                        } else {
                            0
                        },
                    dp[i - 1][j] + 1,
                ),
                dp[i][j - 1] + 1,
            );
        }
    }
    dp[len1 - 1][len2 - 1]
}


// TODO docs
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
            match hashing::get_files_of_directory(PathBuf::from("."), false) {
                Ok(files) => {
                    match files
                    .into_iter()
                        .filter(|x| x.display().to_string().len() > 1)
                    .map(|file| {
                        (
                            file.clone(),
                            string_difference(
                                file.display().to_string(),
                                path.display().to_string(),
                            ),
                        )
                    }).min_by_key(|a| a.1){
                        Some(best_match) if best_match.1 < path.display().to_string().len() / 2 => {
                            let best_match = best_match.0;
                            print!("Did you mean {}? [y/n]", best_match.display());
                            std::io::stdout().flush().expect("Output error");
                            std::io::stdin()
                                .read_line(&mut input)
                                .expect("Can't read input");
                            if input.trim() == "y" {
                                break best_match;
                            }
                        },
                        _ => {}
                    }
                },
                Err(_) => {}
            }
        }
    }
}


// TODO docs
fn get_algorithm() -> (String, String) {
    let mut input = String::new();
    print!("Please put in the hash or leave it empty to compute the hash only: ");
    std::io::stdout().flush().expect("Output error");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can't read input");
    let given_hash: String = input.trim().parse().unwrap();
    match hashing::get_algorithm_by_hash_len(given_hash.len()) {
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
    // TODO documentation
    // TODO readme
    let file = get_path();
    let (algorithm, given_hash) = get_algorithm();
    let computed_hash = hashing::hash(file, &*algorithm).unwrap();
    if given_hash.len() < 2 {
        println!("{}", computed_hash);
    } else if computed_hash == given_hash {
        println!("The hashes are equal");
    } else {
        println!("ERROR:    The hashes are NOT equal");
        println!("Given hash:      {}", given_hash);
        println!("Computed hash:   {}", computed_hash);
    }
}
