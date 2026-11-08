use std::path::{Path, PathBuf};

pub struct Args {
    pub path: PathBuf,
    pub extension: String,
}

impl Args {
    pub fn new(path: &Path, extension: String) -> Args {
        Args {
            path: path.to_path_buf(),
            extension,
        }
    }
}

pub fn parse_args() -> Args {
    let path_string = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let extension_string = std::env::args().nth(2).unwrap_or_else(|| "rs".to_string());

    let path = match path_string == ".".to_string() {
        true => std::env::current_dir().unwrap(),
        false => PathBuf::from(path_string),
    };

    Args::new(&path, extension_string)
}
