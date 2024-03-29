pub mod dts;
pub mod parser;
use serde::{Deserialize, Serialize};

/// A node in CCG derivation tree.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// The name of the rule
    pub rs: parser::ccg::RuleSymbol,
    /// The phonetic form
    pub pf: String,
    /// The syntactic category (in CCG)
    pub cat: parser::ccg::Cat,
    /// The semantic representation (in DTS)
    pub sem: dts::udtt::Preterm,
    /// Signature
    pub sig: Vec<dts::udtt::Signature>,
    /// The daughter nodes
    pub daughters: Vec<Node>,
    /// The score (between 0.00 to 1.00, larger the better)
    pub score: Rational,
    /// The source of the lexical entry
    pub source: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Rational {
    denominator: i64,
    numerator: i64,
}
