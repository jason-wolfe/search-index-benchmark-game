extern crate tantivy;

use tantivy::Index;
use tantivy::query::QueryParser;
use tantivy::tokenizer::TokenizerManager;

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
    let query_parser = QueryParser::new(index.schema(), vec![all_field], TokenizerManager::default());

    let searcher = index.searcher();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let query = query_parser.parse_query(&line).expect("failed to parse query!");
        println!("{}", query.count(&*searcher).expect("Search failed"));
    }

    Ok(())
}