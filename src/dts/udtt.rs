use serde::{Deserialize, Serialize};

/// A type of an element of a type signature, that is, a list of pairs of a preterm and a type.
/// ex. [entity:type, state:type, event:type, student:entity->type]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    text: String,
    type_: Preterm,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "tag", content = "contents")]
pub enum Preterm {
    /// Variables
    Var(i64),
    /// Constant symbols
    Con(String),
    /// The sort \"type\"
    Type,
    /// The sort \"kind\"
    Kind,
    /// Dependent function types (or Pi types)
    Pi(Box<Preterm>, Box<Preterm>),
    /// Negations
    Not(Box<Preterm>),
    /// Lambda abstractions
    Lam(Box<Preterm>),
    /// Function Applications
    App(Box<Preterm>, Box<Preterm>),
    /// Dependent product types (or Sigma types)
    Sigma(Box<Preterm>, Box<Preterm>),
    /// Pairs
    Pair(Box<Preterm>, Box<Preterm>),
    /// (First and second) Projections
    Proj(Selector, Box<Preterm>),
    /// Underspesified terms
    Asp(i64, Box<Preterm>),
    /// Lambda abstractions of a variable vector
    Lamvec(Box<Preterm>),
    /// Function applications of a variable vector
    Appvec(i64, Box<Preterm>),
    /// The unit term (of type Top)
    Unit,
    /// The top type
    Top,
    /// The bottom type
    Bot,
    /// Natural number type (Nat)
    Nat,
    /// 0 (of type Nat)
    Zero,
    /// The successor function
    Succ(Box<Preterm>),
    /// natrec
    Natrec(Box<Preterm>, Box<Preterm>, Box<Preterm>),
    /// Intensional equality types
    Eq(Box<Preterm>, Box<Preterm>, Box<Preterm>),
    /// refl
    Refl(Box<Preterm>, Box<Preterm>),
    /// idpeel
    Idpeel(Box<Preterm>, Box<Preterm>),
    // DRel Int T.Text Preterm Preterm, // Discourse relations
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "tag", content = "contents")]
pub enum Selector {
    Fst,
    Snd,
}
