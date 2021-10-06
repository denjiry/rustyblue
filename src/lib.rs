pub mod dts;
pub mod parser;
use serde::{Deserialize, Serialize};

/// A node in CCG derivation tree.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    rule_symbol: parser::ccg::RuleSymbol,
    phonetic_form: String,
    cat: parser::ccg::Cat,
    semantic: dts::udtt::Preterm,
    signature: Vec<dts::udtt::Signature>,
    daughters: Vec<Node>,
    score: Rational,
    source: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Rational {
    denominator: i32,
    numerator: i32,
}
