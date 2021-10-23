use rustyblue::parser::japanese::lexicon::Lexicon;
use std::io::Result;

use rustyblue::parser::chartparser;
use rustyblue::Node;

fn main() -> Result<()> {
    let input = String::from("ソクラテスは死ぬ。");
    let lex = std::fs::read_to_string("./myLexicon.json")?;
    let nodes: Vec<Node> = serde_json::from_str(&lex)?;
    let lexicon = Lexicon::new(nodes);
    let parsed = chartparser::simple_parse(&input, &lexicon, 10);
    dbg!(parsed.unwrap());
    todo!("patricia map insert delete old entry");
    Ok(())
}
