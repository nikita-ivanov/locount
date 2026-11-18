use std::path::{Path, PathBuf};

pub enum CountMode {
    FAST,
    EXACT,
}

pub struct Args {
    pub path: PathBuf,
    pub count_mode: CountMode,
    pub extension: Option<String>,
    pub exclude_dirs: Vec<String>,
}

impl Args {
    pub fn new(
        path: &Path,
        count_mode: CountMode,
        extension: Option<String>,
        exclude_dirs: Vec<String>,
    ) -> Args {
        Args {
            path: path.to_path_buf(),
            extension,
            count_mode,
            exclude_dirs,
        }
    }
}

pub fn parse_args() -> Args {
    let path_string = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let extension_string = std::env::args().nth(2);
    let count_mode = std::env::args()
        .nth(3)
        .unwrap_or_else(|| "fast".to_string());

    let default_exclude = vec!["target".to_string(), "node_modules".to_string()];

    let exclude_dirs: Vec<String> = match std::env::args().nth(4) {
        Some(dirs) => {
            let mut dirs: Vec<String> = dirs.split(",").map(|s| s.trim().to_string()).collect();
            for dir in default_exclude {
                if !dirs.contains(&dir) {
                    dirs.push(dir);
                }
            }
            // dirs.extend(default_exclude);
            dirs
        }
        None => default_exclude,
    };

    let count_mode = match &count_mode[..] {
        "fast" => CountMode::FAST,
        "exact" => CountMode::EXACT,
        _ => panic!("unknown count mode, available values: fast, exact"),
    };

    let path = match path_string == ".".to_string() {
        true => std::env::current_dir().unwrap(),
        false => PathBuf::from(path_string),
    };

    Args::new(&path, count_mode, extension_string, exclude_dirs)
}
