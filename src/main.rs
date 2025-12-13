mod link;
mod list;
mod parse;

use std::{
    env::{self},
    ffi::OsStr,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use crate::{link::sync_directories, list::ListOperation, parse::normalize_and_extract_codes};

struct Args {
    source: String,
    dest: String,
}

fn main() -> Result<(), ()> {
    // Get command-line arguments
    let raw_args: Vec<String> = env::args().collect();

    if raw_args.len() != 5 {
        print_manual();
        return Err(());
    };

    println!("{:?}", raw_args);

    // Parse interval
    let interval: u64 = match raw_args[3].parse() {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Argument error: {}", e);
            print_manual();
            return Err(());
        }
    };

    // Parse minimum size
    let min_size: u64 = match raw_args[4].parse() {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Argument error: {}", e);
            print_manual();
            return Err(());
        }
    };

    let args = Args {
        source: raw_args[1].to_string(),
        dest: raw_args[2].to_string(),
    };

    let operation = ListOperation {
        source: PathBuf::from(args.source),
        min_size,
    };

    loop {
        let result = operation.initialise(); // Starts the list operation and store in result
        // This result contains the list of files over MIN_SIZE

        match result {
            Ok(vec_of_files) => {
                for i in vec_of_files {
                    let filename = i.file_name().unwrap();
                    let new_name = normalize_and_extract_codes(filename.to_str().unwrap()).unwrap();
                    let extension =
                        OsStr::to_str(Path::new(filename).extension().unwrap()).unwrap();
                    sync_directories(&i, PathBuf::from(&args.dest), new_name.clone(), extension);
                    println!(
                        // Adds the correct extension onto the parsed name
                        "New filename: {}",
                        new_name + "." + extension
                    );
                }
            }
            Err(e) => eprintln!("Error occured when searching directory: {}", e),
        }
        thread::sleep(Duration::from_secs(interval));
    }
}

fn print_manual() {
    println!("jav-parser [SOURCE] [DESTINATION] [INTERVAL] [MINIMUM SIZE IN BYTES]");
}
