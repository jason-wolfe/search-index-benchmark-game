#[macro_use]
extern crate tantivy;
extern crate core;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


use tantivy::schema::SchemaBuilder;
use tantivy::schema::IntOptions;
use tantivy::Index;

use std::env;
use std::io::BufRead;
use std::io::Result;
use std::path::Path;
use tantivy::schema::Cardinality;
use tantivy::schema::TEXT;

fn main() {
    let args: Vec<String> = env::args().collect();
    main_inner(&Path::new(&args[1])).unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct InputDocument {
    url: String,
    title: String,
    body: String,
}

fn main_inner(output_dir: &Path) -> Result<()> {
    let mut schema_builder = SchemaBuilder::default();

    let id_field = schema_builder.add_u64_field("id", IntOptions::default().set_fast(Cardinality::SingleValue));
    let title_field = schema_builder.add_text_field("title", TEXT);
    let all_field = schema_builder.add_text_field("all", TEXT);

    let schema = schema_builder.build();

    let index = Index::create(output_dir, schema).expect("failed to create index");

    // 4 GB heap
    let mut index_writer = index.writer(200_000_000).expect("failed to create index writer");

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }

        let input_doc: InputDocument = serde_json::from_str(&line)?;

        let url_prefix = "https://en.wikipedia.org/wiki?curid=";
        if !input_doc.url.starts_with(url_prefix) {
            continue;
        }
        if let Ok(doc_id) = input_doc.url[url_prefix.len()..].parse::<u64>() {
            let all = format!("{}\n{}", input_doc.title, input_doc.body);
            index_writer.add_document(doc!(
                id_field=>doc_id,
                title_field=>input_doc.title,
                all_field=>all
            ));
        } else {
            println!("invalid doc id in {:?}", input_doc);
        };

    }

    index_writer.commit().expect("failed to commit");
    index_writer.wait_merging_threads().expect("Failed while waiting merging threads");
    Ok(())
}
