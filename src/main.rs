use std;
use std::fs;
use std::env;
use std::path::Path;

use b3rn::WORDS;

use blake3;

fn get_name(data: Vec<u8>, nbytes: usize) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&data);
    let mut output = hasher.finalize_xof();

    let mut bytes = vec![0; nbytes];
    output.fill(&mut bytes);

    let mut name_words: Vec<&str> = Vec::new();
    for byte in bytes {
        name_words.push(WORDS[byte as usize]);
    }

    return name_words.join("_");
}

fn main() {
    let mut args = env::args();
    if args.len() < 3 {
        println!("Usage: b3rn [number of bytes] [files]");
    }

    args.next(); // get rid of the executable filename

    let nbytes = args.next().unwrap()
                          .parse::<usize>()
                          .expect("A number must be provided.");

    for fnstring in args {
        let filename = fnstring.as_str();
        let filedata = match fs::read(filename) {
            Ok(data) => data,
            Err(err) => {
                println!("Error reading {filename}: {err}");
                continue;
            }
        };
    
        let mut new_name = get_name(filedata, nbytes);
        if let Some(extension) = Path::new(filename).extension() {
            new_name += ".";
            new_name += extension.to_str().expect("Non-unicode filename.");
        }

        if let Ok(exists) = Path::new(&new_name).try_exists(){
            if exists {
                println!("File {new_name} already exists!");
                continue;
            }
        }

        match fs::rename(filename, new_name) {
            Ok(_) => {},
            Err(err) => println!("Error renaming {filename}: {err}")
        };
    }
}
