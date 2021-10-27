use crate::parser::ccg::Cat;

/// | Data for category/feature unification
/// csub :: SubstData Cat
/// fsub :: SubstData [FeatureValue]
#[derive(Debug, PartialEq, Eq)]
enum SubstData<T> {
    SubstLink(u32),
    SubstVal(T),
}
type Assignment<T> = Vec<(u32, SubstData<T>)>;

pub(crate) fn unify_category() -> Option<Cat> {
    todo!();
}
