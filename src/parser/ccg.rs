pub mod binary_rules;

use crate::Node;

/// Syntactic categories of
#[derive(Debug, PartialEq)]
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
    Conj,
    /// A category for left parentheses
    Lparen,
    /// A category for right parentheses
    Rparen,
    /// X/Y
    SL(Box<Cat>, Box<Cat>),
    /// X\\Y
    BS(Box<Cat>, Box<Cat>),
    /// Category variables, where Int is an index, Cat is a restriction for its head.
    T(bool, i64, Box<Cat>),
}

#[derive(Debug, PartialEq)]
pub enum Feature {
    ///  Syntactic feature
    F(Vec<FeatureValue>),
    ///  Shared syntactic feature (with an index)
    SF(i64, Vec<FeatureValue>),
}

#[derive(Debug, PartialEq)]
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
    Vsn,
    VZ,
    Vuru,
    Aauo,
    Ai,
    Anas,
    Atii,
    Abes,
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
#[derive(Debug, PartialEq)]
pub enum RuleSymbol {
    /// A lexical item
    Lex,
    /// An empty category
    EC,
    /// Forward function application rule.
    Ffa,
    /// Backward function application rule
    Bfa,
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
    Coord,
    /// Parenthesis rule
    Paren,
    /// Wrap rule
    Wrap,
    /// Dynamic conjunction rule
    DC,
    /// Discourse Relation rule
    Drel,
}
