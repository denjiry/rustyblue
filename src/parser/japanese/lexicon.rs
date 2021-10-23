use patricia_tree::PatriciaMap;

use crate::Node;

pub struct Lexicon {
    map: PatriciaMap<Node>,
}

impl Lexicon {
    /// the time complexity of Lexicon::new() is O({nodes.len()} {max length of *pf*})
    pub fn new(nodes: Vec<Node>) -> Self {
        let map: PatriciaMap<_> = nodes.into_iter().map(|n| (n.pf.clone(), n)).collect();
        Self { map }
    }

    pub fn search(&self, query: &str, offset: usize) -> Option<Vec<Node>> {
        todo!();
    }
}
