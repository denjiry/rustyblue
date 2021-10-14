use patricia_tree::{PatriciaMap, PatriciaSet};
use rustyblue::parser::japanese::lexicon;
use std::io::{Read, Result};
use std::iter::FromIterator;

use rustyblue::parser::chartparser;
use rustyblue::Node;

fn main() -> Result<()> {
    let input = String::from("ソクラテスは死ぬ。");
    let path = std::path::Path::new("./myLexicon.json");
    let mut json = std::fs::File::open(&path)?;
    let mut lex = String::new();
    json.read_to_string(&mut lex)?;
    let nodes = parse_lexicon(&lex)?;
    let _map: PatriciaMap<_> = nodes.into_iter().map(|n| (n.pf.clone(), n)).collect();
    let parsed = chartparser::simple_parse(&input, &input, 10);
    dbg!(parsed.unwrap());
    Ok(())
}

fn parse_lexicon(lex: &str) -> Result<Vec<Node>> {
    let v: Vec<Node> = serde_json::from_str(lex)?;
    Ok(v)
}
