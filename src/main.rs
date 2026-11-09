use locount::args::parse_args;
use locount::count::{Counts, count_lines};
use locount::discovery::list_files;
use std::{path::PathBuf, thread};

fn main() {
    let args = parse_args();

    let paths = list_files(&args.path, &args.extension);
    let num_files = paths.len();
    println!("Files: {num_files}");

    let num_cores = std::thread::available_parallelism().unwrap().get();

    let chunk_size = (num_files / num_cores).max(num_cores);
    let chunks: Vec<Vec<PathBuf>> = paths.chunks(chunk_size).map(|c| c.to_vec()).collect();
    let num_chunks = chunks.len();
    println!("Chunks: {num_chunks}");

    let comment_marks = comment_strings(&args.extension);
    let block_comments = block_comments(&args.extension);

    let mut threads = Vec::new();
    for chunk in chunks {
        let handle = thread::spawn(move || {
            let mut counts = Counts::empty();
            for path in chunk {
                counts = counts + count_lines(&path, comment_marks, block_comments);
            }
            counts
        });
        threads.push(handle);
    }

    let mut counts = Counts::empty();
    for thread in threads {
        let c = thread.join().unwrap();
        counts = counts + c;
    }

    println!("Counts: {counts:?}");

    let total_lines = counts.total();
    println!("Total lines of code: {total_lines}");
}

fn comment_strings(extension: &str) -> &'static [&'static str] {
    match extension {
        "rs" => &["//"],
        "csharp" => &["//"],
        "js" => &["//"],
        "py" => &["#"],
        _ => &[],
    }
}

fn block_comments(extension: &str) -> Option<(&'static str, &'static str)> {
    match extension {
        "rs" => None,
        "csharp" => Some(("/*", "*/")),
        "js" => Some(("/*", "*/")),
        "py" => None,
        _ => None,
    }
}
