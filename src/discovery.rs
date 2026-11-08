use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn list_files(path: &Path, extension: &str) -> Vec<PathBuf> {
    let mut directories = Vec::from([path.to_path_buf()]);

    if path.is_file() {
        return directories;
    }

    let mut files = Vec::new();

    while !&directories.is_empty() {
        let current_dir = directories.pop().unwrap();

        let current_dir = match fs::read_dir(&current_dir) {
            Ok(d) => d,
            Err(e) => {
                // eprintln!("error readong dir {current_dir:?}, will skip. error was: {e:?}");
                continue;
            }
        };

        for item in current_dir {
            let item = match item {
                Ok(x) => x,
                Err(e) => {
                    // eprintln!("error will skip. error was: {e:?}");
                    continue;
                }
            };

            let path = item.path();

            let Ok(file_type) = item.file_type() else {
                // eprintln!("Failed to get file type for {item:?}, will skip");
                continue;
            };

            if file_type.is_file() {
                match path.extension() {
                    Some(ext) => {
                        if ext == extension {
                            files.push(path);
                        }
                    }
                    None => {}
                }
            } else if file_type.is_dir() {
                let path_string = path.clone().into_string().unwrap();
                if path_string.contains(".") || path_string.contains("target/") {
                    continue;
                }
                directories.push(path);
            } else {
                // eprintln!("Unknown file type for {item:?}: {file_type:?}, will skip");
                continue;
            }
        }
    }

    files
}
