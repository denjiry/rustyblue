use crate::Node;

pub struct Lexicon<'a> {
    todo: &'a [u8],
}

impl<'a> Lexicon<'a> {
    pub fn new(lexicon: &'a [u8]) -> Self {
        todo!();
    }

    pub fn search(&self, query: &str, offset: usize) -> Option<Vec<Node>> {
        todo!();
    }
}
