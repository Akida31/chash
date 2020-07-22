mod hash;

use crate::hash::{algorithm_available, get_algorithm, hash};
use std::io::Write;
use std::path::Path;

fn main() {
    let mut input = String::new();
    let file = loop {
        print!("Please put in the path: ");
        std::io::stdout().flush().expect("Output error");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Can't read input");
        input = input.trim().parse().unwrap();
        let path = Path::new(&input);
        if path.is_file() {
            break path;
        }
        else {
            println!("path {:?} not found", path);
        }
    };
    let mut input = String::new();
    print!("Please put in the hash or leave it empty to compute the hash only: ");
    std::io::stdout().flush().expect("Output error");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can't read input");
    let given_hash: String = input.trim().parse().unwrap();
    let mut input = String::new();
    let algorithm = match get_algorithm(given_hash.clone()) {
        Some(a) => a,
        None => loop {
            print!("Please put in the algorithm: ");
            std::io::stdout().flush().expect("Output error");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Can't read input");
            let input: String = input.trim().parse().unwrap();
            if algorithm_available(input.clone()) {
                break input;
            }
            else {
                println!("Algorithm not available");
                // TODO print available algorithms
            }
        },
    };
    let computed_hash = hash(file, algorithm);
    if given_hash.len() < 2 {
        println!("{}", computed_hash);
    } else {
        if computed_hash == given_hash {
            println!("The hashes are equal");
        } else {
            println!("ERROR\nThe hashes are NOT equal");
        }
    }
}
