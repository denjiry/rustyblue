use crate::Node;

pub(crate) type Lexicon<'a> = std::collections::HashMap<&'a str, Vec<&'a Node>>;

pub(crate) fn setup_lexicon(sentence: &str) -> Lexicon {
    unimplemented!()
}
