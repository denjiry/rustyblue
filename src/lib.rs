pub mod dts;
pub mod parser;

/// A node in CCG derivation tree.
#[derive(Debug, PartialEq)]
pub struct Node {
    rule_symbol: parser::ccg::RuleSymbol,
    phonetic_form: String,
    cat: parser::ccg::Cat,
    semantic: dts::udtt::Preterm,
    signature: Vec<dts::udtt::Signature>,
    daughters: Vec<Node>,
    score: f64,
    source: String,
}
