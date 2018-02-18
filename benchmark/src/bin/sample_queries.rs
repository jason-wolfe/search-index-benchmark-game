extern crate csv;
extern crate rand;

use std::env;
use std::fs::File;
use std::io::Result;
use std::path::Path;

use csv::Reader;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let num_queries = args[2].parse::<usize>().expect("invalid num queries");
    main_inner(&Path::new(&args[1]), num_queries).unwrap()
}

fn main_inner(input_path: &Path, num_queries: usize) -> Result<()> {
    let mut reader = Reader::from_reader(File::open(input_path)?);
    let mut weighted_options = Vec::new();
    for record in reader.records() {
        let record = record?;
        weighted_options.push(Weighted {
            weight: record[1].parse::<u32>().expect("encountered invalid weight"),
            item: record[0].to_string(),
        });
    }

    let wc = WeightedChoice::new(&mut weighted_options);
    let mut rng = rand::thread_rng();

    for _ in 0..num_queries {
        let sample = wc.ind_sample(&mut rng);
        println!("{}", sample);
    }

    Ok(())
}