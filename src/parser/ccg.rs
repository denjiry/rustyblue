// Syntactic categories of
#[derive(Debug, PartialEq)]
pub(crate) enum Cat {
    S(Vec<Feature>),        // S
    NP(Vec<Feature>),       // NP
    N,                      // N
    Sbar(Vec<Feature>),     // S bar
    Conj,                   // CON
    Lparen,                 // A category for left parentheses
    Rparen,                 // A category for right parentheses
    SL(Box<Cat>, Box<Cat>), // X/Y
    BS(Box<Cat>, Box<Cat>), // X\\Y
    T(bool, i64, Box<Cat>), // Category variables, where Int is an index, Cat is a restriction for its head.
}

#[derive(Debug, PartialEq)]
pub(crate) enum Feature {
    F(Vec<FeatureValue>),       //  Syntactic feature
    SF(i64, Vec<FeatureValue>), //  Shared syntactic feature (with an index)
}

#[derive(Debug, PartialEq)]
pub(crate) enum FeatureValue {
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

// The name of the CCG rule to derive the node.
#[derive(Debug, PartialEq)]
pub(crate) enum RuleSymbol {
    Lex,   // A lexical item
    EC,    // An empty category
    Ffa,   // Forward function application rule.
    Bfa,   // Backward function application rule
    FFC1,  // Forward function composition rule 1
    BFC1,  // Backward function composition rule 1
    FFC2,  // Forward function composition rule 2
    BFC2,  // Backward function composition rule 2
    FFC3,  // Forward function composition rule 3
    BFC3,  // Backward function composition rule 3
    FFCx1, // Forward function crossed composition rule 1
    FFCx2, // Forward function crossed composition rule 2
    FFSx,  // Forward function crossed substitution rule
    Coord, // Coordination rule
    Paren, // Parenthesis rule
    Wrap,  // Wrap rule
    DC,    // Dynamic conjunction rule
    Drel,  // Discourse Relation rule
}
