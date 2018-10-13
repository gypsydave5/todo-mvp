extern crate reqwest;
extern crate select;
extern crate scraper;

use select::document::Document;
use select::predicate::Name;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use scraper::{Html, Selector};

pub fn main() {

}

fn golden_master_html() -> Html {
    let mut file = File::open("./golden_master.html").unwrap();

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents);

    Html::parse_document(&file_contents)
}

fn actual_website_html() -> Html {
    let mut response = reqwest::get("http://localhost:3000").unwrap();
    let body = response.text().unwrap();

    Html::parse_document(&body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_is_as_expected() {
        let expected = golden_master_html();
        let actual = actual_website_html();
        assert_eq!(expected, actual);
    }
}
