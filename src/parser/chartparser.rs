use crate::{
    parser::{ccg::binary_rules, japanese::lexicon::Lexicon},
    Node,
};

// Simple parsing function to return just the best node for a given sentence
pub fn simple_parse(
    input: &str,
    lexicon: &[u8],
    beam_width: usize,
) -> Result<Node, std::io::Error> {
    let lexicon = Lexicon::new(lexicon);
    let chart_parser = ChartParser {
        beam_width,
        lexicon,
    };
    let chart = chart_parser.parse(input);
    unimplemented!();
}

// removes occurrences of non-letters from an input text.
fn purify_text() {
    unimplemented!();
}

type Chart<'a> = std::collections::HashMap<(usize, usize), Vec<&'a Node>>;

struct ChartParser<'a> {
    beam_width: usize,
    lexicon: Lexicon<'a>,
}

impl<'a> ChartParser<'a> {
    fn parse(&self, sentence: &str) -> Chart {
        let sentence = sentence.chars().collect::<Vec<_>>();
        let mut chart = Chart::new();
        let mut sep_stack = vec![0usize];

        for (i, c) in sentence.iter().enumerate() {
            let sub_sentence = &sentence[..i];
            let j = i + 1;
            for i in (0..i).rev() {
                let mut new_nodes: Vec<Node> = Vec::new();
                let word_candidate: String = sub_sentence[i..].iter().collect();
                let lexes = self.lexicon.search(word_candidate.as_str(), i);
                if let Some(lexes) = lexes {
                    new_nodes.extend(lexes);
                }

                let new_binary_nodes = apply_binary_rules(i, j, &chart);
                new_nodes.extend(new_binary_nodes);

                let old = chart.insert((i, j), new_nodes);
                assert!(old.is_none(), "chart should be inserted up to once.");
            }
        }

        chart
    }
}

fn apply_binary_rules<'a>(i: usize, j: usize, chart: &Chart) -> Vec<Node> {
    use binary_rules::*;
    let mut nodes = Vec::new();
    for k in (i + 1)..j {
        let ik_nodes = match chart.get(&(i, k)) {
            Some(ik_nodes) => ik_nodes,
            None => continue,
        };
        let kj_nodes = match chart.get(&(k, j)) {
            Some(kj_nodes) => kj_nodes,
            None => continue,
        };

        for lnode in ik_nodes {
            for rnode in kj_nodes {
                nodes.push(forward_function_crossed_substitution_rule(lnode, rnode));
            }
        }
    }

    nodes
}
