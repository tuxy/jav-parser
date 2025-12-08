use std::{fs, os::unix::fs::symlink, path::PathBuf};

pub fn sync_directories(
    source: &PathBuf,
    target_folder: &PathBuf,
    filename: String,
    extension: &str,
) {
    // Iterate over entries in source dir
    let target_folder_with_name = target_folder.join(&filename); // Create a folder first (e.g ../ABC-123/ABC-123.mp4)
    // .join(filename + "." + extension); // ...and then symlink the file

    fs::create_dir_all(&target_folder_with_name).unwrap();
    match symlink(
        source,
        &target_folder_with_name.join(filename + "." + extension),
    ) {
        Ok(_) => println!("Linked {:?} to destination", source),
        Err(_) => {} // Ignore errors when symlinking
    };
}
