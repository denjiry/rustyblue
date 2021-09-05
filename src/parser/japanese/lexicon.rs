use crate::Node;
use sudachi::{dic::lexicon::word_infos::WordInfo, tokenizer::Tokenizer};

pub struct Lexicon<'a> {
    todo: &'a [u8],
    sudachi_tokenizer: Tokenizer<'a>,
}

impl<'a> Lexicon<'a> {
    pub fn new(lexicon: &'a [u8]) -> Self {
        let tokenizer = Tokenizer::new(lexicon);
        todo!();
    }

    pub fn search(&self, query: &str, offset: usize) -> Option<Vec<Node>> {
        let sudachi_lexicon = &self.sudachi_tokenizer.lexicon;
        let candidates = sudachi_lexicon.lookup(query.as_bytes(), offset);
        if candidates.len() == 0 {
            return None;
        }
        let mut l: Vec<Node> = vec![];
        for (word_id, _) in candidates {
            let word_info = sudachi_lexicon.get_word_info(word_id as usize);
            l.push(word_info2node(word_info));
        }
        Some(l)
    }
}

fn word_info2node(word_info: WordInfo) -> Node {
    todo!();
}
