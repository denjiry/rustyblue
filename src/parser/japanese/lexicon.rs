use patricia_tree::PatriciaMap;

use crate::Node;

pub struct Lexicon {
    map: PatriciaMap<Vec<Node>>,
}

impl Lexicon {
    /// the time complexity of Lexicon::new() is O({nodes.len()} {max length of *pf*})
    pub fn new(nodes: Vec<Node>) -> Self {
        let mut map: PatriciaMap<Vec<Node>> = PatriciaMap::new();
        for n in nodes {
            let key = n.pf.clone();
            if let Some(v) = map.get_mut(&key) {
                v.push(n);
            } else {
                map.insert(key, vec![n]);
            }
        }
        Self { map }
    }

    pub fn search(&self, query: &str) -> Option<&Vec<Node>> {
        self.map.get(query)
    }
}
