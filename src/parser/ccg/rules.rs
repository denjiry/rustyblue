use crate::parser::ccg::unify;
use crate::Node;

pub fn forward_function_application_rule<'a>(lnode: &Node, rnode: &Node) -> Option<&'a Node> {
    let (_, csub, fsub) = unify::unify_category(vec![], vec![], vec![], &lnode.cat, &rnode.cat)?;
    todo!();
}

pub fn backward_function_application_rule<'a>(lnode: &Node, rnode: &Node) -> Option<&'a Node> {
    todo!();
}
