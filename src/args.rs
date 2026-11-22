use std::path::{Path, PathBuf};

const HELP_STRING: &str = "Usage: locount [--mode | -m] [--extension] [--exclude] [path] ";

#[derive(Clone, Copy, Debug)]
pub enum CountMode {
    FAST,
    EXACT,
}

#[derive(Debug)]
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

#[derive(PartialEq, Eq)]
enum State {
    Start,
    ExpectingMode,
    ExpectingExtension,
    ExpectingExcludeDirs,
}

pub fn parse_args(args: &mut std::env::Args) -> Args {
    let mut state = State::Start;

    // skip the first argument which is the path of the binary
    let _ = args.next();

    let mut path: Option<PathBuf> = None;
    let mut mode: Option<CountMode> = None;
    let mut extension: Option<String> = None;
    let mut exclude_dirs: Option<Vec<String>> = None;

    for arg in args {
        match state {
            State::Start => match arg.as_str() {
                "--help" => {
                    println!("{}", HELP_STRING);
                    std::process::exit(0);
                }
                "--mode" | "-m" => {
                    state = State::ExpectingMode;
                }
                "--extension" => {
                    state = State::ExpectingExtension;
                }
                "--exclude" => {
                    state = State::ExpectingExcludeDirs;
                }
                _ => {
                    if arg.starts_with("-") {
                        eprintln!("Invalid positional argument: {arg}");
                        eprintln!("{}", HELP_STRING);
                        std::process::exit(1);
                    }

                    path = Some(PathBuf::from(&arg));
                }
            },
            State::ExpectingMode => {
                match arg.as_str() {
                    "fast" => {
                        mode = Some(CountMode::FAST);
                    }
                    "exact" => {
                        mode = Some(CountMode::EXACT);
                    }
                    _ => {
                        eprintln!("Unknown --mode argument: {arg}. Supported values: fast, exact.");
                        std::process::exit(1);
                    }
                }
                state = State::Start;
            }
            State::ExpectingExtension => {
                if arg.starts_with("-") {
                    eprintln!("Invalid value parameter value {arg}");
                }
                extension = Some(arg.to_string());
                state = State::Start;
            }
            State::ExpectingExcludeDirs => {
                if arg.starts_with("-") {
                    eprintln!("Invalid value parameter value {arg}");
                }

                exclude_dirs = Some(arg.split(",").map(|s| s.to_string()).collect());
                state = State::Start;
            }
        }
    }
    if state != State::Start {
        eprintln!("Missing value for last flag.");
        eprintln!("{}", HELP_STRING);
        std::process::exit(1);
    }

    let mut exclude_dirs = exclude_dirs.unwrap_or_default();
    exclude_dirs.extend(["target".to_string(), "node_modules".to_string()]);

    Args {
        path: path.unwrap_or_else(|| std::env::current_dir().unwrap()),
        count_mode: mode.unwrap_or(CountMode::FAST),
        extension,
        exclude_dirs,
    }
}
