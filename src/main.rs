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
    let args = parse_args(std::env::args().skip(1));

    let count_lines: fn(&str, &Language) -> Counts = match args.count_mode {
        CountMode::Fast => count_lines_fast,
        CountMode::Exact => count_lines_exact,
    };

    let paths = list_files(&args.path, args.extension.as_deref(), &args.exclude_dirs);
    let num_files = paths.len();
    println!("Files: {num_files}");

    let num_cores = match std::thread::available_parallelism() {
        Ok(res) => res.get(),
        Err(_) => 1,
    };

    let chunk_size = (num_files / num_cores).max(num_cores);
    let chunks: Vec<Vec<(PathBuf, OsString)>> =
        paths.chunks(chunk_size).map(|c| c.to_vec()).collect();

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

                let c = count_lines(&contents, lang);
                counts.push((c, extension));
            }
            counts
        });

        threads.push(handle);
    }

    let mut counts_by_extension: HashMap<OsString, ExtensionAggregate> = HashMap::new();

    for thread in threads {
        let result = thread.join().unwrap();
        for (count, extension) in result {
            counts_by_extension
                .entry(extension)
                .and_modify(|c| {
                    *c = c.add_count(&count);
                })
                .or_insert(ExtensionAggregate::from_count(count));
        }
    }

    let mut counts_vec = counts_by_extension.iter().collect::<Vec<_>>();

    // sort by total line counts
    counts_vec.sort_by_key(|k| k.1.count.total());

    print_results(&counts_vec);
}

#[derive(Debug)]
struct ExtensionAggregate {
    count: Counts,
    num_files: usize,
}

impl ExtensionAggregate {
    fn from_count(count: Counts) -> ExtensionAggregate {
        ExtensionAggregate {
            count,
            num_files: 1,
        }
    }
    fn add_count(&mut self, count: &Counts) -> ExtensionAggregate {
        ExtensionAggregate {
            count: self.count.add(count),
            num_files: self.num_files + 1,
        }
    }
}

fn print_results(results: &Vec<(&OsString, &ExtensionAggregate)>) {
    println!(
        "{:<10} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10}",
        "", "Files", "Total", "Code", "Comments", "Empty", "Code %", "Comments %", "Empty %"
    );

    println!("{}", "-".repeat(111));

    for (extension, aggregate) in results {
        let count = &aggregate.count;
        let total = count.total();
        let percent_code = (count.code as f64) / (total as f64) * 100.0;
        let percent_comments = (count.comments as f64) / (total as f64) * 100.0;
        let percent_empty = (count.empty as f64) / (total as f64) * 100.0;

        println!(
            "{:<10} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10.1} | {:<10.1} | {:<10.1}",
            extension.to_str().unwrap(),
            aggregate.num_files,
            count.total(),
            count.code,
            count.comments,
            count.empty,
            percent_code,
            percent_comments,
            percent_empty
        );
    }
}
