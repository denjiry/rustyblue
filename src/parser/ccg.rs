pub mod rules;
pub mod unify;
use crate::Node;
use serde::{Deserialize, Serialize};

/// Syntactic categories of
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "tag", content = "contents")]
pub enum Cat {
    /// S
    S(Vec<Feature>),
    /// NP
    NP(Vec<Feature>),
    /// N
    N,
    /// S bar
    Sbar(Vec<Feature>),
    /// CON
    CONJ,
    /// A category for left parentheses
    LPAREN,
    /// A category for right parentheses
    RPAREN,
    /// X/Y
    SL(Box<Cat>, Box<Cat>),
    /// X\\Y
    BS(Box<Cat>, Box<Cat>),
    /// Category variables, where Int is an index, Cat is a restriction for its head.
    T(bool, i64, Box<Cat>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "tag", content = "contents")]
pub enum Feature {
    ///  Syntactic feature
    F(Vec<FeatureValue>),
    ///  Shared syntactic feature (with an index)
    SF(i64, Vec<FeatureValue>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FeatureValue {
    V5k,
    V5s,
    V5t,
    V5n,
    V5m,
    V5r,
    V5w,
    V5g,
    V5z,
    V5b,
    V5IKU,
    V5YUK,
    V5ARU,
    V5NAS,
    V5TOW,
    V1,
    VK,
    VS,
    VSN,
    VZ,
    VURU,
    Aauo,
    Ai,
    ANAS,
    ATII,
    ABES,
    Nda,
    Nna,
    Nno,
    Ntar,
    Nni,
    Nemp,
    Nto,
    Exp,
    // Error,
    Stem,
    UStem,
    NStem,
    Neg,
    Cont,
    Term,
    Attr,
    Hyp,
    Imper,
    Pre,
    NTerm,
    NegL,
    TeForm,
    NiForm,
    EuphT,
    EuphD,
    ModU,
    ModD,
    ModS,
    ModM,
    VoR,
    VoS,
    VoE,
    P,
    M,
    Nc,
    Ga,
    O,
    Ni,
    To,
    Niyotte,
    No,
    ToCL,
    YooniCL,
    Decl,
}

/// The name of the CCG rule to derive the node.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RuleSymbol {
    /// A lexical item
    LEX,
    /// An empty category
    EC,
    /// Forward function application rule.
    FFA,
    /// Backward function application rule
    BFA,
    /// Forward function composition rule 1
    FFC1,
    /// Backward function composition rule 1
    BFC1,
    /// Forward function composition rule 2
    FFC2,
    /// Backward function composition rule 2
    BFC2,
    /// Forward function composition rule 3
    FFC3,
    /// Backward function composition rule 3
    BFC3,
    /// Forward function crossed composition rule 1
    FFCx1,
    /// Forward function crossed composition rule 2
    FFCx2,
    /// Forward function crossed substitution rule
    FFSx,
    /// Coordination rule
    COORD,
    /// Parenthesis rule
    PAREN,
    /// Wrap rule
    WRAP,
    /// Dynamic conjunction rule
    DC,
    /// Discourse Relation rule
    DREL,
}
