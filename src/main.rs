use locount::args::{CountMode, parse_args};
use locount::count::{Counts, Language, count_lines_exact, count_lines_fast};
use locount::discovery::list_files;
use std::collections::HashMap;
use std::ffi::OsString;
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
    let chunks: Vec<Vec<(PathBuf, OsString)>> =
        paths.chunks(chunk_size).map(|c| c.to_vec()).collect();

    println!("Chunks: {}", chunks.len());

    let lang = language(&args.extension);

    let mut threads = Vec::new();
    for chunk in chunks {
        let handle = thread::spawn(move || {
            let mut counts: Vec<(Counts, OsString)> = Vec::new();
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
            }
            counts
        });
        threads.push(handle);
    }

    let mut counts_by_extension: HashMap<OsString, Counts> = HashMap::new();

    for thread in threads {
        let result = thread.join().unwrap();
        for (count, extension) in result {
            // TODO: not sure this is idiomatic approach to modify hashmap value
            let empty = Counts::empty();
            let current_count = counts_by_extension.get(&extension).unwrap_or(&empty);
            counts_by_extension.insert(extension, current_count.add(&count));
        }
    }

    let mut counts_vec = counts_by_extension.iter().collect::<Vec<_>>();
    counts_vec.sort_by_key(|k| k.1.total());

    for (extension, count) in counts_vec {
        println!("Counts {extension:?}: {count:?}");
    }
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
