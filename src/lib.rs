pub mod dts;
pub mod parser;
use serde::{Deserialize, Serialize};

/// A node in CCG derivation tree.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// The name of the rule
    rs: parser::ccg::RuleSymbol,
    /// The phonetic form
    pf: String,
    /// The syntactic category (in CCG)
    cat: parser::ccg::Cat,
    /// The semantic representation (in DTS)
    sem: dts::udtt::Preterm,
    /// Signature
    sig: Vec<dts::udtt::Signature>,
    /// The daughter nodes
    daughters: Vec<Node>,
    /// The score (between 0.00 to 1.00, larger the better)
    score: Rational,
    /// The source of the lexical entry
    source: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Rational {
    denominator: i64,
    numerator: i64,
}
