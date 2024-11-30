use std::{collections::HashMap, path::Path};

use clap::Parser;
use display::print_results;
use ignore::WalkBuilder;
use itertools::Itertools;
use lang::Language;

mod display;
mod lang;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Show all found languages
    #[arg(short, long, conflicts_with = "count")]
    all: bool,

    /// Number of entries to show
    #[arg(short = 'n', long, default_value_t = 10, conflicts_with = "all")]
    count: usize,

    /// List recognized files and their results
    #[arg(short, long)]
    debug: bool,

    /// Path to search
    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let pwd = match args.path {
        Some(p) => Path::new(&p).canonicalize(),
        None => std::env::current_dir(),
    };
    let pwd = match pwd {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let builder = WalkBuilder::new(pwd.clone());
    let walk = builder.build();

    let mut results: HashMap<Language, usize> = HashMap::new();
    for entry in walk {
        match entry {
            Err(e) => eprintln!("Error: {}", e),
            Ok(entry) => {
                if entry.file_type().unwrap().is_file() {
                    let Some(lang) = lang::determine_language(entry.path().to_path_buf()) else {
                        if args.debug {
                            eprintln!("{}: unrecognized file", entry.path().to_string_lossy())
                        }
                        continue;
                    };
                    let sloc = read_sloc(entry.path());

                    if args.debug {
                        eprintln!(
                            "{}: {}, {} lines",
                            entry.path().to_string_lossy(),
                            lang,
                            sloc
                        )
                    }

                    if let std::collections::hash_map::Entry::Vacant(e) = results.entry(lang) {
                        e.insert(sloc);
                    } else {
                        *results.get_mut(&lang).unwrap() += sloc;
                    }
                }
            }
        }
    }

    let results = results.into_iter().sorted_by(|a, b| Ord::cmp(&b.1, &a.1));

    let results: Vec<_> = if args.all {
        results.collect()
    } else {
        results.take(args.count).collect()
    };

    print_results(results, &pwd);
}

fn read_sloc(path: &Path) -> usize {
    let mut sloc = 0;
    for line in std::fs::read_to_string(path).unwrap().lines() {
        if !line.is_empty() {
            sloc += 1
        }
    }

    sloc
}
