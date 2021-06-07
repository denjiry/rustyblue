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
        let chart = chart_parser(beam, &lexicon, &self.sentence);
        unimplemented!();
    }

    // removes occurrences of non-letters from an input text.
    fn purify_text(&mut self) {
        unimplemented!();
    }
}

type Chart = std::collections::HashMap<(usize, usize), Vec<Node>>;
fn chart_parser(beam: usize, lexicon: &LexicalItem, sentence: &str) -> Chart {
    let chars = sentence.chars().fuse();
    let initial_pc = PartialChart::new();
    let end = chars.fold(initial_pc, |acc, c| {
        chart_accumulator(beam, lexicon, acc, c)
    });
    end.chart
}

struct PartialChart {
    chart: Chart,
    seplist: Vec<usize>,
    i: usize,
    stack: String,
}
impl PartialChart {
    fn new() -> Self {
        PartialChart {
            chart: Chart::new(),
            seplist: vec![],
            i: 0,
            stack: String::new(),
        }
    }
}
fn chart_accumulator(
    beam: usize,
    lexicon: &LexicalItem,
    pc: PartialChart,
    c: char,
) -> PartialChart {
    unimplemented!();
}
