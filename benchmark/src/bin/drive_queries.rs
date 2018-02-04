extern crate stopwatch;

use std::env;
use std::fs;
use std::io::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

use stopwatch::Stopwatch;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    main_inner(&Path::new(&args[1])).unwrap()
}

fn main_inner(base_dir: &Path) -> Result<()> {
    let num_repeats = 20;

    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    let mut result_buf = String::new();

    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
            eprintln!("Executing file at {:?}", path);
            let file = BufReader::new(File::open(path)?);
            for line in file.lines() {
                let line = line?;

                if line.len() > 0 {
                    eprintln!("query: {}", line);
                    let overall_stopwatch = Stopwatch::start_new();
                    for _ in 0..num_repeats {
                        let query_stopwatch = Stopwatch::start_new();
                        result_buf.clear();
                        println!("{}", line);
                        stdin_handle.read_line(&mut result_buf)?;
                        let elapsed = query_stopwatch.elapsed_ms();
                        let num_results: u64 = result_buf.trim_right().parse().map_err(|_| format!("invalid u64 response: {}", result_buf)).unwrap();
                        eprintln!("num_results = {}, elapsed = {}ms", num_results, elapsed);
                    }

                    let average_time = overall_stopwatch.elapsed_ms() as f64 / num_repeats as f64;
                    eprintln!("average_time = {:?}", average_time);
                }
            }
        }
    }

    Ok(())
}