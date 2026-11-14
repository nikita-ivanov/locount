use locount::args::{CountMode, parse_args};
use locount::count::{Counts, Language, count_lines_exact, count_lines_fast};
use locount::discovery::list_files;
use std::collections::BTreeMap;
use std::fs;
use std::{path::PathBuf, thread};

fn main() {
    let args = parse_args();

    let count_lines: fn(&str, &Language) -> Counts = match args.count_mode {
        CountMode::FAST => count_lines_fast,
        CountMode::EXACT => count_lines_exact,
    };

    let paths = list_files(&args.path, &args.extension);
    let num_files = paths.len();
    println!("Files: {num_files}");

    let num_cores = std::thread::available_parallelism().unwrap().get();

    let chunk_size = (num_files / num_cores).max(num_cores);
    let chunks: Vec<Vec<(PathBuf, String)>> =
        paths.chunks(chunk_size).map(|c| c.to_vec()).collect();
    let num_chunks = chunks.len();
    println!("Chunks: {num_chunks}");

    let lang = language(&args.extension);

    let mut threads = Vec::new();
    for chunk in chunks {
        let handle = thread::spawn(move || {
            let mut counts: Vec<(Counts, String)> = Vec::new();
            for (path, extension) in chunk {
                let contents = match fs::read_to_string(&path) {
                    Ok(res) => res,
                    Err(_) => {
                        // eprintln!("Error reading file {path:?}: {e:?}");
                        continue;
                    }
                };
                let c = count_lines(&contents, &lang);
                counts.push((c, extension));
                // counts = counts + count_lines(&contents, &lang);
            }
            counts
        });
        threads.push(handle);
    }

    let mut counts_by_extension: BTreeMap<String, Counts> = BTreeMap::new();
    for thread in threads {
        let result = thread.join().unwrap();
        for (count, extension) in result {
            let empty = Counts::empty();
            let current_count = counts_by_extension.get(&extension).unwrap_or(&empty);
            counts_by_extension.insert(extension, current_count.add(&count));
        }
    }

    for (extension, count) in counts_by_extension {
        println!("Counts {extension}: {count:?}");
    }

    // let total_lines = counts.total();
    // println!("Total lines of code: {total_lines}");
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
