use super::japanese::lexicon::setup_lexicon;
use crate::{parser::japanese::lexicon, Node};
use lexicon::LexicalItem;

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
            let stack = &sentence[..i];
        }

        chart
    }
}
