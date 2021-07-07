use super::japanese::lexicon::setup_lexicon;
use crate::{parser::japanese::lexicon, Node};
use lexicon::LexicalItem;

const MAX_WORD_LENGTH: usize = 23;

struct Input {
    sentence: String,
}

impl Input {
    // Simple parsing function to return just the best node for a given sentence
    fn simple_parse(&self, beam_width: usize) -> Result<Node, std::io::Error> {
        let lexicon = setup_lexicon(&self.sentence);
        let chart_parser = ChartParser {
            beam_width,
            lexicon,
        };
        let chart = chart_parser.parse(&self.sentence);
        unimplemented!();
    }

    // removes occurrences of non-letters from an input text.
    fn purify_text(&mut self) {
        unimplemented!();
    }
}

type Chart = std::collections::HashMap<(usize, usize), Vec<Node>>;

struct ChartParser {
    beam_width: usize,
    lexicon: LexicalItem,
}

impl ChartParser {
    fn parse(&self, sentence: &str) -> Chart {
        let sentence = sentence.chars().collect::<Vec<_>>();
        let mut chart = Chart::new();
        let mut sep_stack = vec![0usize];

        for (i, c) in sentence.iter().enumerate() {
            let sub_sentence = &sentence[..i];
            let j = i + 1;
            let backward_lex_seeker = (0..i).rev();
            for i in backward_lex_seeker {
                let word_candidate = &sub_sentence[i..];
                if word_candidate.len() > MAX_WORD_LENGTH {
                    continue;
                }
                let lexes = self.lexicon.lookup(word_candidate);
                let new_parent_nodes = check_binary_rules(i, j, &chart);
                let new_nodes: Vec<Node> = vec![lexes, new_parent_nodes]
                    .into_iter()
                    .flatten()
                    .collect();
                let old = chart.insert((i, j), new_nodes);
                assert!(old.is_none(), "chart should be inserted only once.");
            }
        }

        chart
    }
}

fn check_binary_rules(i: usize, j: usize, chart: &Chart) -> Vec<Node> {
    for k in ((i + 1)..j) {
        let kj_nodes = chart.get(&(k, j));
        let ik_nodes = chart.get(&(i, k));
    }
    todo!();
}
