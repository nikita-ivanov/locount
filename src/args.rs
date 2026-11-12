use std::path::{Path, PathBuf};

pub enum CountMode {
    FAST,
    EXACT,
}

pub struct Args {
    pub path: PathBuf,
    pub extension: String,
    pub count_mode: CountMode,
}

impl Args {
    pub fn new(path: &Path, extension: String, count_mode: CountMode) -> Args {
        Args {
            path: path.to_path_buf(),
            extension,
            count_mode,
        }
    }
}

pub fn parse_args() -> Args {
    let path_string = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let extension_string = std::env::args().nth(2).unwrap_or_else(|| "rs".to_string());
    let count_mode = std::env::args()
        .nth(3)
        .unwrap_or_else(|| "fast".to_string());

    let count_mode = match &count_mode[..] {
        "fast" => CountMode::FAST,
        "exact" => CountMode::EXACT,
        _ => panic!("unknown count mode, available values: fast, exact"),
    };

    let path = match path_string == ".".to_string() {
        true => std::env::current_dir().unwrap(),
        false => PathBuf::from(path_string),
    };

    Args::new(&path, extension_string, count_mode)
}
