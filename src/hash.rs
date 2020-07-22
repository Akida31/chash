use sha1::Digest;
use std::fmt::Write;
use std::fs::File;
use std::path::Path;

fn byte_to_hex(input: &[u8]) -> String {
    let mut out = String::new();
    for &byte in input {
        write!(&mut out, "{:x}", byte).expect("Parsing Error");
    }
    out
}

fn sha1(input: &[u8]) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(input);
    byte_to_hex(&*hasher.finalize())
}

fn sha256(input: &[u8]) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input);
    byte_to_hex(&*hasher.finalize())
}

fn sha384(input: &[u8]) -> String {
    let mut hasher = sha2::Sha384::new();
    hasher.update(input);
    byte_to_hex(&*hasher.finalize())
}

fn sha512(input: &[u8]) -> String {
    let mut hasher = sha2::Sha512::new();
    hasher.update(input);
    byte_to_hex(&*hasher.finalize())
}

fn md5(input: &[u8]) -> String {
    format!("{:?}", md5::compute(input))
}

/// computes the hash of the given path with the given algorithm
/// returns the hexdigits (lowercase) of the hash
/// Path must be a valid path and the algorithm must be one of the following:
///
/// Panics:
/// if path is not a Path
pub fn hash(path: &Path, algorithm: String) -> String {
    // Okay be
    File::open(path).unwrap();
    let algorithm = match &*algorithm {
        "md5" => md5,
        "sha1" => sha1,
        "sha256" => sha256,
        "sha384" => sha384,
        "sha512" => sha512,
        _ => panic!("Algorithm not available!"),
    };
    algorithm(b"Hello")
}

pub fn get_algorithm(hash: String) -> Option<String> {
    println!("{}", hash.len());
    Some(
        match hash.len() {
            32 => "md5",
            40 => "sha1",
            64 => "sha256",
            96 => "sha284",
            128 => "sha512",
            _ => return None,
        }
        .to_string(),
    )
}

pub fn get_all_algorithms() -> Vec<String> {
    vec!["md5", "sha1", "sha256", "sha284", "sha512"].into_iter().map(|x| {
        String::from(x)
    }).collect()
}

pub fn algorithm_available(algorithm: String) -> bool {
    get_all_algorithms().contains(&algorithm)
}
