use patricia_tree::PatriciaMap;
use rustyblue::parser::japanese::lexicon::{self, Lexicon};
use std::{io::Result, iter::FromIterator};

use rustyblue::parser::chartparser;
use rustyblue::Node;

fn main() -> Result<()> {
    let input = String::from("ソクラテスは死ぬ。");
    let path = std::path::Path::new("./myLexicon.json");
    let lex = std::fs::read_to_string("./myLexicon.json")?;
    let nodes = parse_lexicon(&lex)?;
    let lexicon = Lexicon::new(nodes);
    let parsed = chartparser::simple_parse(&input, &lexicon, 10);
    dbg!(parsed.unwrap());
    Ok(())
}

fn parse_lexicon(lex: &str) -> Result<Vec<Node>> {
    let v: Vec<Node> = serde_json::from_str(lex)?;
    Ok(v)
}
