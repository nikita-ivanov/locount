use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

const HELP_STRING: &'static str =
    "Usage: locount [path] [--mode | --count_mode] [--extension] [--exclude]";

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
    Argument,
    Parameter,
    Help,
}

// TODO: should display help string on panic
pub fn parse_args() -> Args {
    let mut state = State::Start;
    let mut builder = ArgsBuilder::new();

    let mut iter = std::env::args();
    while let Some(s) = iter.next() {
        if state != State::Start && s.starts_with("--") {
            state = State::Parameter;
        }

        if s == "--help" {
            state = State::Help;
        }

        match state {
            State::Start => {
                // this is path of the binary, which we don't need
                state = State::Argument;
                continue;
            }
            State::Help => {
                println!("{}", HELP_STRING);
                std::process::exit(0);
            }
            State::Argument => {
                builder.with_path(&s);
            }
            State::Parameter => {
                let mut split = s.split("--");
                split.next().unwrap();

                let param_name = split.next().unwrap();
                println!("param {param_name}");

                let Some(param_value) = iter.next() else {
                    println!("Missing param value for {param_name}");
                    println!("{}", HELP_STRING);
                    std::process::exit(1);
                };

                match param_name {
                    "count_mode" | "mode" => builder.with_count_mode(&param_value),
                    "extension" => builder.with_extension(&param_value),
                    "exclude" => builder.with_exclude_dirs(&param_value),
                    _ => {
                        println!("Unknown parameter {param_name}");
                        println!("{}", HELP_STRING);
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    builder.build()
}

struct ArgsBuilder {
    path: Option<PathBuf>,
    count_mode: Option<CountMode>,
    extension: Option<String>,
    exclude_dir: Option<Vec<String>>,
}

impl ArgsBuilder {
    pub fn new() -> ArgsBuilder {
        ArgsBuilder {
            path: None,
            count_mode: None,
            extension: None,
            exclude_dir: None,
        }
    }
    pub fn with_path(&mut self, path: &str) {
        self.path = Some(PathBuf::from_str(path).unwrap());
    }

    pub fn with_count_mode(&mut self, count_mode: &str) {
        match count_mode {
            "fast" => {
                self.count_mode = Some(CountMode::FAST);
            }
            "exact" => {
                self.count_mode = Some(CountMode::EXACT);
            }
            _ => {
                println!("Unknown count mode supplied: {count_mode}. Will skip.")
            }
        }
    }

    pub fn with_extension(&mut self, extension: &str) {
        self.extension = Some(extension.to_string());
    }

    pub fn with_exclude_dirs(&mut self, dirs: &str) {
        self.exclude_dir = Some(dirs.split(",").map(|s| s.to_string()).collect());
    }

    pub fn build(&self) -> Args {
        let path = self
            .path
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap());

        let count_mode = self.count_mode.unwrap_or_else(|| CountMode::FAST);

        let extension = self.extension.clone();

        let mut exclude_dirs = self.exclude_dir.clone().unwrap_or_else(|| Vec::new());
        exclude_dirs.extend(vec!["target".to_string(), "node_modules".to_string()]);

        Args {
            path,
            count_mode,
            extension,
            exclude_dirs,
        }
    }
}
