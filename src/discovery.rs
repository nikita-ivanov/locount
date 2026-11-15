use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

use crate::language::languages;

const IGNORE_DIRS: &'static [&'static str] = &["target", "node_modules"];

fn path_to_str<'a>(path: &'a Path) -> Option<&'a str> {
    let Some(file_name) = path.file_name() else {
        return None;
    };

    let Some(path_str) = file_name.to_str() else {
        return None;
    };

    Some(path_str)
}

pub fn list_files(path: &Path, extension: Option<&str>) -> Vec<(PathBuf, OsString)> {
    if path.is_file() {
        let Some(ext) = path.extension() else {
            return Vec::new();
        };

        return vec![(path.to_path_buf(), ext.to_owned())];
    }

    let langs = languages();

    let mut directories = Vec::from([path.to_path_buf()]);

    let mut files: Vec<(PathBuf, OsString)> = Vec::new();

    while !&directories.is_empty() {
        let current_dir = directories.pop().unwrap();

        let current_dir = match fs::read_dir(&current_dir) {
            Ok(d) => d,
            Err(_) => {
                // eprintln!("error readong dir {current_dir:?}, will skip. error was: {e:?}");
                continue;
            }
        };

        for item in current_dir {
            let item = match item {
                Ok(x) => x,
                Err(_) => {
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
                let Some(path_str) = path_to_str(&path) else {
                    continue;
                };

                if path_str.starts_with(".") {
                    continue;
                }

                let Some(ext) = path.extension() else {
                    continue;
                };

                match extension {
                    Some(e) => {
                        if e == ext {
                            files.push((path.clone(), ext.to_owned()));
                        }
                    }
                    None => {
                        if langs.iter().any(|l| l.0 == &ext) {
                            files.push((path.clone(), ext.to_owned()))
                        }
                    }
                }
            } else if file_type.is_dir() {
                let Some(path_str) = path_to_str(&path) else {
                    continue;
                };

                if path_str.starts_with(".") || IGNORE_DIRS.iter().any(|d| d == &path_str) {
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
