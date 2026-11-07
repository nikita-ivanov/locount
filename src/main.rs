use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    ops::Add,
    path::PathBuf,
    thread,
};

#[derive(Debug)]
struct Counts {
    code: usize,
    comments: usize,
    empty: usize,
}

impl Counts {
    fn new() -> Counts {
        Counts {
            code: 0,
            comments: 0,
            empty: 0,
        }
    }

    fn total(&self) -> usize {
        self.code + self.comments + self.empty
    }
}

impl Add for Counts {
    type Output = Counts;

    fn add(self, rhs: Self) -> Self::Output {
        Counts {
            code: self.code + rhs.code,
            comments: self.comments + rhs.comments,
            empty: self.empty + rhs.empty,
        }
    }
}

fn main() {
    // let path = PathBuf::from("/Users/nivanov/code/rust/kraken_l3");
    let path = PathBuf::from("/Users/nivanov/code/rust/zed");
    // let path = PathBuf::from("/Users/nivanov/code/rust/zed-forks/goldstrikearch/zed");
    // let path = PathBuf::from("/Users/nivanov/code/rust/gram");

    let paths = list_files(path);
    let n = paths.len();
    println!("Total of {n} files");

    let num_cores = std::thread::available_parallelism().unwrap().get();
    println!("num cores: {num_cores}");
    let chunk_size = n / num_cores;
    let chunks: Vec<Vec<PathBuf>> = paths.chunks(chunk_size).map(|c| c.to_vec()).collect();
    let num_chunks = chunks.len();
    println!("Total {num_chunks} chunks");

    let mut handles = Vec::new();
    for chunk in chunks {
        let handle = thread::spawn(move || {
            let mut counts = Counts::new();
            for path in chunk {
                counts = counts + count_lines3(&path);
            }
            counts
        });
        handles.push(handle);
    }

    let mut counts = Counts::new();
    for h in handles {
        let c = h.join().unwrap();
        counts = counts + c;
    }

    println!("totals: {counts:?}");

    let total_lines = counts.total();
    println!("Total lines of code: {total_lines}");
}

fn count_lines(path: &PathBuf) -> Counts {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut total_code = 0;
    let mut total_comments = 0;
    let mut total_empty = 0;
    for line in reader.lines() {
        match line {
            Ok(text) => {
                if text.starts_with("//") {
                    total_comments += 1;
                } else if text.is_empty() {
                    total_empty += 1;
                } else {
                    total_code += 1;
                }
            }
            Err(e) => {
                // eprintln!("Error reading line, will skip. error was: {e:?}");
                continue;
            }
        }
    }

    Counts {
        code: total_code,
        comments: total_comments,
        empty: total_empty,
    }
}

fn count_lines2(path: &PathBuf) -> Counts {
    let file = File::open(path).unwrap();
    let map = unsafe { memmap2::Mmap::map(&file).unwrap() };

    let mut total_code = 0;
    let mut total_comments = 0;
    let mut total_empty = 0;
    for line in map.split(|c| *c == b'\n') {
        if line.is_empty() {
            total_empty += 1;
        } else if line.len() > 1 && line[..2] == *"//".as_bytes() {
            total_comments += 1;
        } else {
            total_code += 1;
        }
    }

    Counts {
        code: total_code,
        comments: total_comments,
        empty: total_empty,
    }
}

fn count_lines3(path: &PathBuf) -> Counts {
    let contents = fs::read_to_string(path).unwrap();

    let mut total = 0;
    let mut total_comments = 0;
    let mut total_empty = 0;

    for line in contents.split("\n") {
        if line.starts_with("//") {
            total_comments += 1;
        } else if line.is_empty() {
            total_empty += 1;
        } else {
            total += 1;
        }
    }

    Counts {
        code: total,
        comments: total_comments,
        empty: total_empty,
    }
}
fn list_files(path: PathBuf) -> Vec<PathBuf> {
    let mut directories = Vec::from([path.clone()]);

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
                        if ext == "rs" {
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

            //             let metadata = match item.metadata() {
            //                 Ok(m) => m,
            //                 Err(e) => {
            //                     // eprintln!("could not read metadata for {path:?}, will skip. error was: {e:?}");
            //                     continue;
            //                 }
            //             };
            //
            //             if metadata.is_file() {
            //                 match path.extension() {
            //                     Some(ext) => {
            //                         if ext == "rs" {
            //                             files.push(path);
            //                         }
            //                     }
            //                     None => {}
            //                 }
            //             } else {
            //                 let path_string = path.clone().into_string().unwrap();
            //                 if path_string.contains(".") || path_string.contains("target/") {
            //                     continue;
            //                 }
            //                 directories.push(path);
            //             }
        }
    }

    files
}
