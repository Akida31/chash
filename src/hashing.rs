use std::fmt::Write;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use sha1::Digest;
use std::fs;

/// The size of the buffer reading the file
const BUFFER_SIZE: usize = 1024;

/// Get a List of all supported algorithms
pub fn available_algorithms() -> [&'static str; 5] {
    ["md5", "sha1", "sha256", "sha384", "sha512"]
}

/// Evaluates the algorithm from the length of a hash
///
/// # Examples
/// ```
/// assert_eq!(get_algorithm_by_hash_len(32), Some(String::from("md5"))
/// assert_eq!(get_algorithm_by_hash_len(17), None)
/// ```
pub fn get_algorithm_by_hash_len(len: usize) -> Option<String> {
    Some(
        match len {
            32 => "md5",
            40 => "sha1",
            64 => "sha256",
            96 => "sha384",
            128 => "sha512",
            _ => return None,
        }
        .to_string(),
    )
}

/// Converts a byte-sequence to a human readable format.
///
/// The output consists of lowercase hexadecimal numbers.
///
/// # Examples
/// ```
/// // some bytes in a array
/// let input: &[u8] = &[240, 159, 146, 32];
/// assert_eq!(byte_to_hex(input), String::from("hiasdf"))
/// ```
fn byte_to_hex(input: &[u8]) -> String {
    let mut out = String::new();
    for &byte in input {
        write!(&mut out, "{:02x}", byte).expect("Parsing Error");
    }
    out
}

/// Lists all files of the given directory
///
/// It returns a Vec of PathBufs for every file.
/// The recursive option can be used to get also all files of subdirectories recursively.
///
/// # Examples
/// ```
/// use std::io::Result;
/// use std::path::PathBuf;
///
/// // get all files of the current folder
/// let files: Result<Vec<PathBuf>> = get_files_of_directory(PathBuf::from("."), false);
/// ```
pub fn get_files_of_directory(direcory: PathBuf, recursive: bool) -> std::io::Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(direcory)? {
        let path = entry?.path();
        if path.is_dir() && recursive {
            result.append(&mut get_files_of_directory(path, recursive).unwrap());
        } else {
            result.push(path);
        }
    }
    Ok(result)
}

/// computes the hash of the given path with the given algorithm
///
/// returns the hexdigits (lowercase) of the hash
/// Path must be a valid path.
/// If the Path is a directory, all subdirecories with their files will be hashed
///
/// # Errors:
/// if `path` is not a valid path
/// if `algorithm` is not a supported algorithm, get a list of all by calling `available_algorithms`
///
/// # Example:
/// hash a file
/// ```
/// use std::path::Pathbuf;
/// let file = PathBuf::from("./Cargo.toml");
/// let algorithm = "sha256";
/// let result = hash(file, &algorithm);
/// ```
pub fn hash(path: PathBuf, algorithm: &str) -> Result<String, String> {
    if !available_algorithms().contains(&algorithm) {
        return Err(format!("{} is not a available algorithm!", algorithm));
    }
    let mut hasher: Box<dyn digest::DynDigest> = match algorithm {
        "md5" => Box::new(md5::Md5::new()),
        "sha1" => Box::new(sha1::Sha1::new()),
        "sha256" => Box::new(sha2::Sha256::new()),
        "sha384" => Box::new(sha2::Sha384::new()),
        "sha512" => Box::new(sha2::Sha512::new()),
        a => unimplemented!("Algorithm {} is not implemented", a),
    };
    let files = if path.is_file() {
        vec![path]
    } else {
        get_files_of_directory(path, true).unwrap()
    };
    for file in files {
        let file = match File::open(&file) {
            Ok(f) => f,
            Err(_) => return Err(format!("{:?} is not a valid path!", file)),
        };
        let mut reader = BufReader::new(file);
        let mut buffer = [0u8; BUFFER_SIZE];
        while let Ok(n) = reader.read(&mut buffer) {
            hasher.update(&buffer[..n]);
            if n == 0 || n < BUFFER_SIZE {
                break;
            }
        }
    }
    Ok(byte_to_hex(&*hasher.finalize()))
}
