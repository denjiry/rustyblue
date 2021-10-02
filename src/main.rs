use std::io::{Read, Result};

use rustyblue::parser::chartparser;
use serde_json::Value;

fn main() -> Result<()>{
    let input = String::from("ソクラテスは死ぬ。");
    let path = std::path::Path::new("./myLexicon.json");
    let mut json =  std::fs::File::open(&path)?;
    let mut lex = String::new();
    json.read_to_string(&mut lex)?;
    parse_lexicon(&lex)?;
    let node = chartparser::simple_parse(&input, &input, 10);
    dbg!(node.unwrap());
    Ok(())
}

fn parse_lexicon(lex: &str) -> Result<()>{
    let v: Value = serde_json::from_str(lex)?;
    dbg!(&v[0]);
    Ok(())
}
