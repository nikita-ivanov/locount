use locount::count::{Counts, count_lines};
use locount::discovery::list_files;
use std::{path::PathBuf, thread};

fn main() {
    let path_string = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let extension_string = std::env::args().nth(2).unwrap_or_else(|| "rs".to_string());

    let path = match path_string == ".".to_string() {
        true => std::env::current_dir().unwrap(),
        false => PathBuf::from(path_string),
    };

    let paths = list_files(path, &extension_string);
    let num_files = paths.len();
    println!("Files: {num_files}");

    let num_cores = std::thread::available_parallelism().unwrap().get();
    println!("Cores: {num_cores}");
    let chunk_size = (num_files / num_cores).max(num_cores);
    let chunks: Vec<Vec<PathBuf>> = paths.chunks(chunk_size).map(|c| c.to_vec()).collect();
    let num_chunks = chunks.len();
    println!("Chunks: {num_chunks}");

    let mut handles = Vec::new();
    for chunk in chunks {
        let handle = thread::spawn(move || {
            let mut counts = Counts::empty();
            for path in chunk {
                counts = counts + count_lines(&path);
            }
            counts
        });
        handles.push(handle);
    }

    let mut counts = Counts::empty();
    for h in handles {
        let c = h.join().unwrap();
        counts = counts + c;
    }

    println!("Counts: {counts:?}");

    let total_lines = counts.total();
    println!("Total lines of code: {total_lines}");
}
