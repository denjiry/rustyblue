use crate::Node;
use sudachi::tokenizer::Tokenizer;

pub(crate) type Lexicon<'a> = std::collections::HashMap<&'a str, Vec<&'a Node>>;

pub(crate) fn setup_lexicon(sentence: &str) -> Lexicon {
    unimplemented!()
}
