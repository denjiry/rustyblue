use super::japanese::lexicon::setup_lexicon;
use crate::{parser::japanese::lexicon, Node};
use lexicon::LexicalItem;

struct Input {
    sentence: String,
}

impl Input {
    // Simple parsing function to return just the best node for a given sentence
    fn simple_parse(&self, beam: usize) -> Result<Node, std::io::Error> {
        let lexicon = setup_lexicon(&self.sentence);
        let chart = chart_parser(lexicon, &self.sentence);
        unimplemented!();
    }

    // removes occurrences of non-letters from an input text.
    fn purify_text(&mut self) {
        unimplemented!();
    }
}

type Chart = std::collections::HashMap<(usize, usize), Vec<Node>>;
fn chart_parser(lexicon: LexicalItem, sentence: &str) -> Chart {
    unimplemented!();
}

struct PartialChart {
    chart: Chart,
    seplist: Vec<usize>,
    i: usize,
    stack: String,
}
fn chart_accumulator(beam: usize, lexicon: LexicalItem, pc: PartialChart, c: char) -> PartialChart {
    unimplemented!();
}
