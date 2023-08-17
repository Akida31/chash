use std::cmp::{max, min};
use std::io::Write;
use std::path::PathBuf;

mod hashing;

/// Compute the difference between the filenames
///
/// If the first string starts with the second string the return value is 0
/// else the return value is the number of different characters between the two strings
/// beginning => 0
///
///
/// # Examples
/// ```
/// assert_eq!(get_best_match(String::from("Hello"), String::from("World")), 4);
/// assert_eq!(get_best_match(String::from("aaa"), String::from("bbb")), 3);
/// assert_eq!(get_best_match(String::from("Hello World"), String::from("Hello")), 0);
/// ```
fn get_best_match(str1: String, str2: String) -> usize {
    if str1.starts_with(&str2) {
        return 0;
    }
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

/// get the path of the file or directory which should be checked from userinput
///
/// returns the Path of the file/ directory
fn get_path() -> PathBuf {
    loop {
        let mut input = String::new();
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
            if let Ok(files) = hashing::get_files_of_directory(PathBuf::from("."), false) {
                if let Some(best_match) = files
                    .into_iter()
                    .filter(|x| x.display().to_string().len() > 1)
                    .map(|file| {
                        (
                            file.clone(),
                            get_best_match(
                                file.display()
                                    .to_string()
                                    .trim_start_matches("./")
                                    .to_string(),
                                path.display().to_string(),
                            ),
                        )
                    })
                    .min_by_key(|a| a.1)
                {
                    if best_match.1 <= path.display().to_string().len() / 5 {
                        let best_match = best_match.0.display().to_string();
                        print!(
                            "Did you mean {}? [y/n] ",
                            best_match.trim_start_matches("./")
                        );
                        let mut input = String::new();
                        std::io::stdout().flush().expect("Output error");
                        std::io::stdin()
                            .read_line(&mut input)
                            .expect("Can't read input");
                        if input.trim() == "y" {
                            break PathBuf::from(best_match);
                        }
                    }
                }
            }
        }
    }
}

/// get the hash from userinput
fn get_hash() -> String {
    let mut input = String::new();
    print!("Please put in the hash or leave it empty to compute the hash only: ");
    std::io::stdout().flush().expect("Output error");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can't read input");
    input.trim().parse().unwrap()
}

/// get the algorithm from userinput
///
/// The algorithm can also be computed if the size of the hash given by the user has a unambiguous length
fn get_algorithm(given_hash: Option<String>) -> String {
    let mut given_hash = given_hash;
    if given_hash.is_some() {
        given_hash = hashing::get_algorithm_by_hash_len(given_hash.unwrap().len());
    }
    match given_hash {
        Some(a) => a,
        None => loop {
            let mut input = String::new();
            print!("Please put in the algorithm: ");
            std::io::stdout().flush().expect("Output error");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Can't read input");
            let algorithm: String = input.trim().parse().unwrap();
            if hashing::available_algorithms().contains(&&*algorithm) {
                break algorithm;
            } else {
                println!("Algorithm not available\nAvailable algorithms:");
                for algo in hashing::available_algorithms().iter() {
                    println!("  {}", algo);
                }
            }
        },
    }
}

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("path")
                .help("The path of the file or directory which should be hashed")
                .short("p")
                .long("path")
                .takes_value(true)
                .validator(|path| {
                    let path = PathBuf::from(path);
                    if path.is_file() || path.is_dir() {
                        Ok(())
                    } else {
                        Err("Path not found".to_string())
                    }
                }),
        )
        .arg(
            clap::Arg::with_name("algorithm")
                .help("The algorithm which should be used")
                .short("a")
                .long("algorithm")
                .takes_value(true)
                .possible_values(&hashing::available_algorithms()),
        )
        .arg(
            clap::Arg::with_name("hash")
                .help("The hash which should be checked")
                .short("h")
                .long("hash")
                .takes_value(true),
        )
        .get_matches();
    let path = match matches.value_of("path") {
        Some(path) => PathBuf::from(path),
        None => get_path(),
    };
    let given_hash = match matches.value_of("hash") {
        Some(hash) => hash.to_string(),
        None => get_hash(),
    };
    let algorithm = match matches.value_of("algorithm") {
        Some(algo) => algo.to_string(),
        None => get_algorithm(Some(given_hash.clone())),
    };
    let computed_hash = hashing::hash(path, &*algorithm).unwrap();
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
