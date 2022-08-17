use rustyblue::parser::japanese::lexicon::Lexicon;
use std::env::set_var;
use std::fmt::format;
use std::io::{ErrorKind, Result};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

use rustyblue::parser::{chartparser, lightblue};
use rustyblue::Node;

fn main() -> Result<()> {
    let input = String::from("黄色い大きな花は枯れる。");
    // let lex = std::fs::read_to_string("./myLexicon.json")?;
    // let nodes: Vec<Node> = serde_json::from_str(&lex)?;
    // let nodes: Vec<&Node> = nodes.iter().collect();
    // let lexicon = Lexicon::new(nodes);
    // let parsed = chartparser::simple_parse(&input, &lexicon, 10)?;
    let parsed = lightblue::lightblue_wrapper(&input)?;
    dbg!(&parsed[0]);
    Ok(())
}
