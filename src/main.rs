use locount::args::parse_args;
use locount::count::{Counts, Language, count_lines};
use locount::discovery::list_files;
use std::fs;
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

    let lang = language(&args.extension);

    let mut threads = Vec::new();
    for chunk in chunks {
        let handle = thread::spawn(move || {
            let mut counts = Counts::empty();
            for path in chunk {
                let contents = fs::read_to_string(&path).unwrap();
                counts = counts + count_lines(&contents, &lang);
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

fn language(extension: &str) -> Language {
    match extension {
        "rs" => Language::new(&["//"], None),
        "py" => Language::new(&["#"], None),
        "js" => Language::new(&["//"], Some(("/*", "*/"))),
        "csharp" => Language::new(&["//"], Some(("/*", "*/"))),
        _ => Language::new(&[], None),
    }
}
