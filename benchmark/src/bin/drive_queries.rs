extern crate clap;
extern crate stopwatch;

use std::env;
use std::fs;
use std::io::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use clap::{Arg, App};
use stopwatch::Stopwatch;

fn main() {
    let matches = App::new("Query Driver")
        .arg(
            Arg::with_name("queries")
                .short("q")
                .long("queries")
                .value_name("FILE|DIRECTORY")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("num_repetitions")
                .short("n")
                .value_name("#repetitions")
                .takes_value(true)
                .default_value("20")
        )
        .get_matches();

    let query_file_path = Path::new(matches.value_of("queries").unwrap());
    let num_repetitions = matches.value_of("num_repetitions").unwrap().parse::<usize>().expect("invalid number of repetitions given");

    main_inner(&query_file_path, num_repetitions).unwrap()
}

fn main_inner(base_dir: &Path, num_repeats: usize) -> Result<()> {
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    process(&mut stdin_handle, base_dir, num_repeats)?;

    Ok(())
}

fn process(input: &mut BufRead, dir: &Path, num_repeats: usize) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            process(input, &entry.path(), num_repeats)?;
        }
    } else if dir.is_file() && dir.extension().map_or(false, |ext| ext == "txt") {
        eprintln!("Executing file at {:?}", dir);
        let file_stopwatch = Stopwatch::start_new();
        let file = BufReader::new(File::open(&dir)?);

        let mut result_buf = String::new();
        let mut num_queries = 0;

        for line in file.lines() {
            let line = line?;

            if line.len() > 0 {
                eprintln!("query: {}", line);
                let overall_stopwatch = Stopwatch::start_new();
                for _ in 0..num_repeats {
                    let query_stopwatch = Stopwatch::start_new();
                    result_buf.clear();
                    println!("{}", line);
                    input.read_line(&mut result_buf)?;
                    let elapsed = query_stopwatch.elapsed_ms();
                    let num_results: u64 = result_buf.trim_right().parse().map_err(|_| format!("invalid u64 response: {}", result_buf)).unwrap();
                    eprintln!("num_results = {}, elapsed = {}ms", num_results, elapsed);
                }

                if num_repeats > 1 {
                    let average_time = overall_stopwatch.elapsed_ms() as f64 / num_repeats as f64;
                    eprintln!("average_time = {:?}", average_time);
                }

                num_queries += 1;
            }
        }

        if num_queries > 1 {
            let file_elapsed = file_stopwatch.elapsed_ms();
            let avg_elapsed = file_elapsed as f64 / num_queries as f64;
            eprintln!("processing {} queries in file {:?} took = {:?}. Average = {}.", num_queries, dir, file_elapsed, avg_elapsed);
        }
    }

    Ok(())
}