use std::{collections::HashMap, hash::Hash};

pub fn counts<K: Eq + Hash, I: IntoIterator<Item = K>>(i: I) -> HashMap<K, usize> {
    let iter = i.into_iter();
    let (lower, _) = iter.size_hint();
    let mut counts = HashMap::with_capacity(lower);
    for k in iter {
        *counts.entry(k).or_insert(0) += 1;
    }
    counts
}
