use std;
use std::fs;
use std::env;
use std::path::Path;
use blake3;

fn main() {
    let mut args = env::args();
    if args.len() < 3 {
        println!("Usage: b3rn [number of bytes] [files]");
    }

    args.next(); // get rid of the filename

    let bytes = args.next().unwrap().parse::<usize>().expect("A number must be provided.");
    for fnstring in args {
        let filename = fnstring.as_str();
        let filedata = match fs::read(filename) {
            Ok(data) => data,
            Err(err) => {
                println!("Error reading {filename}: {err}");
                continue;
            }
        };

        let mut hasher = blake3::Hasher::new();
        hasher.update(&filedata);
        let mut output = hasher.finalize_xof();

        let mut bName = vec![0; bytes];
        output.fill(&mut bName);

        let mut new_name = String::new();
        for byte in bName {
            new_name += &format!("{:02x}", byte);
        }
    
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
