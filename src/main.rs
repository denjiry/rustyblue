use rustyblue::parser::japanese::lexicon::Lexicon;
use std::io::{ErrorKind, Result};

use rustyblue::parser::chartparser;
use rustyblue::Node;

fn main() -> Result<()> {
    let input = String::from("ソクラテスは死ぬ。");
    let lex = std::fs::read_to_string("./myLexicon.json")?;
    let nodes: Vec<Node> = serde_json::from_str(&lex)?;
    let nodes: Vec<&Node> = nodes.iter().collect();
    let lexicon = Lexicon::new(nodes);
    let parsed = chartparser::simple_parse(&input, &lexicon, 10)?;
    dbg!(parsed);
    Ok(())
}
