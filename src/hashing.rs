use std::fmt::Write;
use std::fs::File;
use std::io::{BufReader, Read, Write as IoWrite};
use std::path::PathBuf;

use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
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
/// let result = hash(file, &algorithm, None);
/// ```
pub fn hash(
    path: PathBuf,
    algorithm: &str,
    output_individual: Option<PathBuf>,
) -> Result<String, String> {
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
    let (individual_hasher, mut individual_file) =
        if let Some(output_individual) = &output_individual {
            let mut file = File::create(output_individual)
                .map_err(|e| format!("can't open file for output-individual: {}", e))?;
            file.write_all(
                format!(
                    "hash algorithm: {}\nbase: {}\n----------\n",
                    algorithm,
                    path.to_string_lossy()
                )
                .as_bytes(),
            )
            .map_err(|e| format!("can't write header to output-individual: {}", e))?;
            (Some(hasher.clone()), Some(file))
        } else {
            (None, None)
        };

    let files = if path.is_file() {
        vec![path]
    } else {
        let mut files = get_files_of_directory(path, true).unwrap();
        files.sort();
        files
    };

    let progresses = MultiProgress::new();
    let pb_style = ProgressStyle::with_template(
        "[{elapsed_precise}] [{bar:.cyan/blue}] {bytes:>10}/{total_bytes:>10} (ETA: {eta}) ({msg})",
    )
    .unwrap()
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
        write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
    })
    .progress_chars("#>-");
    let n_files = files.len() as u64;
    let file_pb = progresses.add(ProgressBar::new(n_files)
        .with_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{bar:.cyan/blue}] hashed {pos:>4}/{len:>4} files (ETA: {eta})"
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-")));

    for (i, filepath) in files.into_iter().enumerate() {
        if let Some(path) = &output_individual {
            if path
                .canonicalize()
                .map(|path| filepath.canonicalize().unwrap() == path)
                .unwrap_or_else(|_| &filepath == path)
            {
                println!("skipping output-individual file {}", path.to_string_lossy());
                continue;
            }
        }
        let mut file_hasher = individual_hasher.clone();
        file_pb.set_position(i as u64);
        let filepath_str = filepath.to_string_lossy();
        let filename_str = filepath
            .file_name()
            .map(|s| s.to_string_lossy())
            .unwrap_or_else(|| filepath_str.clone());
        let file = match File::open(&filepath) {
            Ok(f) => f,
            Err(e) => return Err(format!("{} is not a valid path: {}", filepath_str, e)),
        };
        let pb = {
            let total_size = file
                .metadata()
                .map_err(|e| format!("can't read metadata for {}: {}", filepath_str, e))?
                .len();
            let pb = progresses.add(ProgressBar::new(total_size));
            pb.set_style(pb_style.clone());
            pb.set_message(format!("hashing {}", filename_str));

            pb
        };

        let mut reader = BufReader::new(file);
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut position = 0;
        while let Ok(n) = reader.read(&mut buffer) {
            file_pb.tick();
            pb.set_position(position as u64);
            hasher.update(&buffer[..n]);
            if let Some(file_hasher) = file_hasher.as_mut() {
                file_hasher.update(&buffer[..n]);
            }
            position += n;

            if n == 0 || n < BUFFER_SIZE {
                pb.finish_and_clear();
                break;
            }
        }
        if let Some(file) = individual_file.as_mut() {
            file.write_all(
                format!(
                    "- {} = {}\n",
                    filepath_str,
                    byte_to_hex(&file_hasher.unwrap().finalize())
                )
                .as_bytes(),
            )
            .map_err(|e| format!("can't write line to output-individual: {}", e))?;
        }
    }
    file_pb.finish_with_message(format!("hashed {} files", n_files));
    let hash = byte_to_hex(&hasher.finalize());
    if let Some(file) = individual_file.as_mut() {
        file.write_all(format!("----------\ncomplete hash = {}\n", hash).as_bytes())
            .map_err(|e| format!("can't write final line to output-individual: {}", e))?;
    }

    Ok(hash)
}
