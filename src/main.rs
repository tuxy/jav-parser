mod link;
mod list;
mod parse;

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use crate::{link::sync_directories, list::ListOperation, parse::normalize_and_extract_codes};

const SOURCE_DIR: &str = "source_dir";
const DEST_DIR: &str = "dest_dir";
const MIN_SIZE: u64 = 400000000;
const INTERVAL: u64 = 20;

fn main() {
    let operation = ListOperation {
        source: PathBuf::from(SOURCE_DIR),
        min_size: MIN_SIZE,
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
                    sync_directories(&i, &PathBuf::from(DEST_DIR), new_name.clone(), extension);
                    println!(
                        // Adds the correct extension onto the parsed name
                        "New filename: {}",
                        new_name + "." + extension
                    );
                }
            }
            Err(e) => eprintln!("Error occured when searching directory: {}", e),
        }
        thread::sleep(Duration::from_secs(INTERVAL));
    }
}
