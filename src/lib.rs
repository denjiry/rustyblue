mod ccg;
mod udtt;

// A node in CCG derivation tree.
#[derive(Debug, PartialEq)]
struct Node {
    rule_symbol: ccg::RuleSymbol,
    phonetic_form: String,
    cat: ccg::Cat,
    semantic: udtt::Preterm,
    signature: Vec<udtt::Signature>,
    daughters: Vec<Node>,
    score: f64,
    source: String,
}
