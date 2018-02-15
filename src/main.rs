extern crate hyper;
extern crate select;

use select::document::Document;
use select::predicate::{Class,Name};
use select::node::Node;

struct Article {
    title:   String,
    link:    String,
    details: String,
    summary: String,
}

impl Article {
    fn get_articles() -> Vec<Article> {
    let document = Document::from(open_testing());
    document.find(Name("article")).map(|node| Article::new(&node)).collect()
            
    }
    fn new(node: &Node) -> Article {
        let header = node.find(Name("a")).nth(0).unwrap();
        let mut link = String::from(header.attr("href").unwrap());
        if link.starts_with("/") { assert_eq!(link.remove(0), '/'); }
        let mut details = node.find(Class("details")).nth(0).unwrap().text();
        if details.contains("Add A Comment") {
            details = details.replace("Add A Comment", "0 Comments");
        }
        let summary = node.find(Name("p")).nth(0).unwrap().text();
        Article {
            title: header.text(),
            link: link,
            details: details,
            summary: summary,
        }
    }


}

fn open_testing() -> &'static str {
    include_str!("phoronix.html")
}

fn main() {
    let phoronix_articles = Article::get_articles();
    for article in phoronix_articles.iter().rev() {
        println!("Title:   {}", article.title);
        println!("Link:    https://www.phoronix.com/{}", article.link);
        println!("Details: {}", article.details);
        println!("Summary: {}\n", article.summary);
    }
}