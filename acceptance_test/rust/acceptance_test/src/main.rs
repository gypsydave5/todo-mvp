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

fn golden_master_page() -> Html {
    let mut file = File::open("./golden_master.html").unwrap();

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents);

    Html::parse_document(&file_contents)
}

fn page() -> Html {
    let mut response = reqwest::get("http://localhost:3000").unwrap();
    let body = response.text().unwrap();

    Html::parse_document(&body)
}

fn add_a_todo(name: &str) {
    let params = [("item", name)];
    let client = client();

    client.post("http://localhost:3000")
        .form(&params)
        .send()
        .unwrap();
}


fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .redirect(reqwest::RedirectPolicy::none())
        .build()
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_is_as_expected() {
        let expected = golden_master_page();
        let actual = page();
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_add_a_todo() {
        add_a_todo("tony");
        let page = page();
    }
}
