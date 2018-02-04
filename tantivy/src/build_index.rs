extern crate tantivy;
extern crate core;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


use tantivy::schema::Schema;
use tantivy::schema::SchemaBuilder;
use tantivy::schema::IntOptions;
use tantivy::schema::TextOptions;
use tantivy::schema::TextIndexingOptions;
use tantivy::Index;

use std::io::BufRead;
use std::io::Result;
use std::path::Path;
use tantivy::schema::Document;

fn main() {
    main_inner().unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct InputDocument {
    url: String,
    title: String,
    body: String,
}

fn main_inner() -> Result<()> {
    let mut schema_builder = SchemaBuilder::default();

    let id = schema_builder.add_u64_field("id", IntOptions::default().set_fast());
    let title = schema_builder.add_text_field("title", TextOptions::default().set_indexing_options(TextIndexingOptions::TokenizedWithFreqAndPosition).set_stored());
    let all = schema_builder.add_text_field("all", TextOptions::default().set_indexing_options(TextIndexingOptions::TokenizedWithFreqAndPosition));

    let schema = schema_builder.build();

    let index = Index::create(Path::new("/tmp/wiki_index"), schema).expect("failed to create index");

    // 4 GB heap
    let mut index_writer = index.writer(500_000_000).expect("failed to create index writer");

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
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
