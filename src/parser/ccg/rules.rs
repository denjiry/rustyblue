use crate::Node;
use crate::parser::ccg::unify;

pub fn forward_function_application_rule<'a>(lnode: &Node, rnode: &Node) -> Option<&'a Node> {
    let new_category = unify::unify_category();
    todo!();
}

pub fn backward_function_application_rule<'a>(lnode: &Node, rnode: &Node) -> Option<&'a Node> {
    todo!();
}
