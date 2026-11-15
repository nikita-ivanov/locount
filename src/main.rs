use locount::Language;
use locount::args::{CountMode, parse_args};
use locount::count::{Counts, count_lines_exact, count_lines_fast};
use locount::discovery::list_files;
use locount::language::languages;
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

    let paths = list_files(&args.path, args.extension.as_deref());
    let num_files = paths.len();
    println!("Files: {num_files}");

    let num_cores = std::thread::available_parallelism().unwrap().get();

    let chunk_size = (num_files / num_cores).max(num_cores);
    let chunks: Vec<Vec<(PathBuf, OsString)>> =
        paths.chunks(chunk_size).map(|c| c.to_vec()).collect();

    println!("Chunks: {}", chunks.len());

    let mut threads = Vec::new();
    for chunk in chunks {
        let handle = thread::spawn(move || {
            let langs = languages();
            let mut counts: Vec<(Counts, OsString)> = Vec::new();

            for (path, extension) in chunk {
                let contents = match fs::read_to_string(&path) {
                    Ok(res) => res,
                    Err(_) => {
                        // eprintln!("Error reading file {path:?}: {e:?}");
                        continue;
                    }
                };

                // TODO: is it ok perfomance wise and idiomatic?
                let Some(extension_str) = extension.to_str() else {
                    continue;
                };

                let Some(lang) = langs.get(&extension_str) else {
                    continue;
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
            counts_by_extension
                .entry(extension)
                .and_modify(|c| *c = c.add(&count))
                .or_insert(count);
        }
    }

    let mut counts_vec = counts_by_extension.iter().collect::<Vec<_>>();
    counts_vec.sort_by_key(|k| k.1.total());

    for (extension, count) in counts_vec {
        println!("Counts {extension:?}: {count:?}");
    }
}
