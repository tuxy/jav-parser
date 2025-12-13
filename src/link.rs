use std::{fs, os::unix::fs::symlink, path::PathBuf};

pub fn sync_directories(
    source: &PathBuf,
    target_folder: PathBuf,
    filename: String,
    extension: &str,
) {
    // Iterate over entries in source dir
    let target_folder_with_name = target_folder.join(&filename); // Create a folder first (e.g ../ABC-123/ABC-123.mp4)
    // .join(filename + "." + extension); // ...and then symlink the file

    match fs::create_dir_all(&target_folder_with_name) {
        Ok(_) => {}
        Err(e) => {
            // Permission error, etc...
            eprintln!("Could not create destination folder: {}", e);
            return;
        }
    }
    if symlink(
        source,
        target_folder_with_name.join(filename + "." + extension),
    )
    .is_ok()
    {
        println!("Linked {:?} to destination", source)
    };
}
