extern crate tantivy;

use tantivy::Index;
use tantivy::query::QueryParser;
use tantivy::collector::CountCollector;

use std::env;
use std::io::BufRead;
use std::io::Result;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    main_inner(&Path::new(&args[1])).unwrap()
}

fn main_inner(index_dir: &Path) -> Result<()> {
    let index = Index::open(index_dir).expect("failed to open index");
    let all_field = index.schema().get_field("all").expect("no all field?!");
    let query_parser = QueryParser::new(index.schema(), vec![all_field]);

    let searcher = index.searcher();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;

        let query = query_parser.parse_query(&line).expect("failed to parse query!");
        let mut count_collector = CountCollector::default();
        searcher.search(&*query, &mut count_collector).expect("failed to execute query");
        println!("{}", count_collector.count());
    }

    Ok(())
}