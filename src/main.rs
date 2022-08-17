use rustyblue::parser::japanese::lexicon::Lexicon;
use std::env::set_var;
use std::fmt::format;
use std::io::{ErrorKind, Result};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

use rustyblue::parser::chartparser;
use rustyblue::Node;

fn main() -> Result<()> {
    let input = String::from("ソクラテスは死ぬ。");
    // let lex = std::fs::read_to_string("./myLexicon.json")?;
    // let nodes: Vec<Node> = serde_json::from_str(&lex)?;
    // let nodes: Vec<&Node> = nodes.iter().collect();
    // let lexicon = Lexicon::new(nodes);
    // let parsed = chartparser::simple_parse(&input, &lexicon, 10)?;
    let home = std::env::var("HOME").unwrap();
    set_var("LIGHTBLUE", format!("{}/code/lightblue/", home));
    let mut process = Command::new("lightblue")
        .arg("-s")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let mut stdin = process.stdin.take().unwrap();
        stdin.write_all(input.as_bytes())?;
    }
    let mut stdout = process.stdout.unwrap();
    let mut s = String::new();
    stdout.read_to_string(&mut s)?;
    let parsed: Vec<Node> = serde_json::from_str(&s)?;
    dbg!(&parsed);
    Ok(())
}
