use std::collections::HashMap;
use crate::domain::models::Rule;

pub fn leftcorners_dict<'a>(grammar: &'a [Rule]) -> HashMap<&'a str, Vec<&Rule>> {
    let mut leftcorners = HashMap::new();
    for rule in grammar {
        let entry = leftcorners.entry(rule.rhs[0].as_str()).or_insert(Vec::new());
        entry.push(rule);
    }
    leftcorners
}
