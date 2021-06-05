// A type of an element of a type signature, that is, a list of pairs of a preterm and a type.
// ex. [entity:type, state:type, event:type, student:entity->type]
#[derive(Debug, PartialEq)]
pub(crate) struct Signature {
    text: String,
    type_: Preterm,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Preterm {
    Var(i64),                                         // Variables
    Con(String),                                      // Constant symbols
    Type,                                             // The sort \"type\"
    Kind,                                             // The sort \"kind\"
    Pi(Box<Preterm>, Box<Preterm>),                   // Dependent function types (or Pi types)
    Not(Box<Preterm>),                                // Negations
    Lam(Box<Preterm>),                                // Lambda abstractions
    App(Box<Preterm>, Box<Preterm>),                  // Function Applications
    Sigma(Box<Preterm>, Box<Preterm>),                // Dependent product types (or Sigma types)
    Pair(Box<Preterm>, Box<Preterm>),                 // Pairs
    Proj(Selector, Box<Preterm>),                     // (First and second) Projections
    Asp(i64, Box<Preterm>),                           // Underspesified terms
    Lamvec(Box<Preterm>),                             // Lambda abstractions of a variable vector
    Appvec(i64, Box<Preterm>),                        // Function applications of a variable vector
    Unit,                                             // The unit term (of type Top)
    Top,                                              // The top type
    Bot,                                              // The bottom type
    Nat,                                              // Natural number type (Nat)
    Zero,                                             // 0 (of type Nat)
    Succ(Box<Preterm>),                               // The successor function
    Natrec(Box<Preterm>, Box<Preterm>, Box<Preterm>), // natrec
    Eq(Box<Preterm>, Box<Preterm>, Box<Preterm>),     // Intensional equality types
    Refl(Box<Preterm>, Box<Preterm>),                 // refl
    Idpeel(Box<Preterm>, Box<Preterm>),               // idpeel
                                                      // DRel Int T.Text Preterm Preterm, // Discourse relations
}

#[derive(Debug, PartialEq)]
pub(crate) enum Selector {
    Fst,
    Snd,
}
