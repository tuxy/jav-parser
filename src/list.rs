use std::path::PathBuf;
use walkdir::WalkDir;

pub struct ListOperation {
    pub source: PathBuf,
    pub min_size: u64,
}

impl ListOperation {
    /// Recursively traverses a directory and prints files larger than a specified size.
    pub fn initialise(&self) -> std::io::Result<Vec<PathBuf>> {
        let mut list_of_files: Vec<PathBuf> = Vec::new();
        // WalkDir provides an iterator that recursively traverses the directory tree.
        for entry in WalkDir::new(self.source.clone()) {
            // Handle potential errors when accessing directory entries
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Warning: Could not access file system entry: {}", e);
                    continue; // Skip this entry
                }
            };

            // Get the metadata, which contains file size and type
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(e) => {
                    // Permission was denied
                    eprintln!(
                        "Warning: Could not get metadata for '{}': {}",
                        entry.path().display(),
                        e
                    );
                    continue; // Skip this entry
                }
            };

            // 3. Filter: Check if it's a file and if the size is over the limit
            if metadata.is_file() {
                let file_size = metadata.len();
                if file_size > self.min_size {
                    // 4. Output: Print the human-readable size and file path
                    let human_size = self.bytes_to_human_readable(file_size);
                    println!(
                        "Entry found: {:>10} - {}",
                        human_size,
                        entry.path().display()
                    );
                    list_of_files.push(entry.into_path());
                }
            }
        }

        Ok(list_of_files)
    }

    /// Helper function to convert bytes to a human-readable format (B, KB, MB, GB, etc.).
    fn bytes_to_human_readable(&self, bytes: u64) -> String {
        const UNITS: [&str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

        if bytes < 1024 {
            return format!("{} {}", bytes, UNITS[0]);
        }

        let mut i = 0;
        let mut size = bytes as f64;

        while size >= 1024.0 && i < UNITS.len() - 1 {
            size /= 1024.0;
            i += 1;
        }

        // Use a fixed width for alignment and two decimal places
        format!("{:.2} {}", size, UNITS[i])
    }
}
