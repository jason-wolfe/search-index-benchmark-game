extern crate csv;
extern crate scraper;

use std::env;
use std::fs;
use std::io::Read;
use std::io::Result;
use std::fs::File;
use std::path::Path;

use csv::Writer;
use scraper::{Html, Selector};

/// Extract pairs of search topics and counts from data in the format of the
/// Google Search Trends web page.
/// Output a CSV of these pairs.

fn main() {
    let args: Vec<String> = env::args().collect();
    main_inner(&Path::new(&args[1]), &Path::new(&args[2])).unwrap()
}

fn main_inner(html_file: &Path, output_path: &Path) -> Result<()> {
    let mut file = File::open(html_file)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let fragment = Html::parse_fragment(&file_contents);
    let container_selector = Selector::parse(".hottrends-trends-list-trend-container").expect("failed to parse selector");
    let title_selector = Selector::parse(".hottrends-single-trend-title").expect("failed to parse selector");
    let count_selector = Selector::parse(".hottrends-single-trend-info-line-number").expect("failed to parse selector");

    let mut writer = Writer::from_path(output_path)?;

    for container in fragment.select(&container_selector) {
        if let Some(elem) = container.select(&title_selector).next() {
            let title = elem.text().collect::<String>();
            if let Some(elem) = container.select(&count_selector).next() {
                let raw_count = elem.text().collect::<String>();
                let count = raw_count.chars().filter(|x| x.is_ascii_digit()).collect::<String>();
                writer.write_record(&[title, count]);
            }
        }
    }

    Ok(())

}