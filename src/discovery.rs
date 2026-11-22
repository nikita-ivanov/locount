use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

use crate::language::languages;

fn path_to_file_name(path: &Path) -> Option<&str> {
    let file_name = path.file_name()?;
    let file_name_str = file_name.to_str()?;
    Some(file_name_str)
}

pub fn list_files(
    path: &Path,
    extension: Option<&str>,
    exclude_dirs: &[String],
) -> Vec<(PathBuf, OsString)> {
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
                continue;
            }
        };

        for item in current_dir {
            let Ok(item) = item else {
                continue;
            };

            let path = item.path();

            let Ok(file_type) = item.file_type() else {
                continue;
            };

            if file_type.is_file() {
                let Some(file_name) = path_to_file_name(&path) else {
                    continue;
                };

                if file_name.starts_with(".") {
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
                let Some(dir_name) = path_to_file_name(&path) else {
                    continue;
                };

                if dir_name.starts_with(".") || exclude_dirs.iter().any(|d| d == dir_name) {
                    continue;
                }

                directories.push(path);
            } else {
                continue;
            }
        }
    }

    files
}
