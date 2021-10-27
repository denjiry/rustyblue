use crate::parser::ccg::{Cat, FeatureValue};

/// Data for category/feature unification
/// csub :: SubstData Cat
/// fsub :: SubstData [FeatureValue]
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SubstData<T> {
    SubstLink(u32),
    SubstVal(T),
}
pub(crate) type Assignment<T> = Vec<(u32, SubstData<T>)>;

pub(crate) fn unify_category(
    csub: Assignment<Cat>,
    fsub: Assignment<Vec<FeatureValue>>,
    banned: Vec<u32>,
    cat1: &Cat,
    cat2: &Cat,
) -> Option<(Cat, Assignment<Cat>, Assignment<Vec<FeatureValue>>)> {
    todo!();
}
