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
use tantivy::schema::Document;
use tantivy::schema::Cardinality;
use tantivy::schema::{TEXT, STORED};

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

    let id = schema_builder.add_u64_field("id", IntOptions::default().set_fast(Cardinality::SingleValue));
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let all = schema_builder.add_text_field("all", TEXT);

    let schema = schema_builder.build();

    let index = Index::create(output_dir, schema).expect("failed to create index");

    // 4 GB heap
    let mut index_writer = index.writer(500_000_000).expect("failed to create index writer");

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }

        let input_doc: InputDocument = serde_json::from_str(&line)?;

        let url_prefix = "https://en.wikipedia.org/wiki?curid=";
//        eprintln!("doc = {:?}", input_doc);
        if !input_doc.url.starts_with(url_prefix) {
            continue;
        }
        if let Ok(doc_id) = input_doc.url[url_prefix.len()..].parse::<u64>() {
            let mut doc = Document::default();
            doc.add_u64(id, doc_id);
            doc.add_text(title, &input_doc.title);
            doc.add_text(all, &format!("{}\n{}", input_doc.title, input_doc.body));

            let id = index_writer.add_document(doc);
        } else {
            println!("invalid doc id in {:?}", input_doc);
        };

    }

    index_writer.commit().expect("failed to commit");

    Ok(())
}
