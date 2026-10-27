use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    rc::Rc,
};

use serde::{Deserialize, Serialize};

use crate::metric::hamming_metric;

#[derive(Debug, Clone)]
pub struct HeapWord {
    parent: Option<Rc<HeapWord>>,
    word: String,
    total_cost: Reverse<u32>,
}
impl Eq for HeapWord {}
impl PartialEq for HeapWord {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost == other.total_cost
    }
}

impl PartialOrd for HeapWord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total_cost.partial_cmp(&other.total_cost)
    }
}

impl Ord for HeapWord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_cost.cmp(&other.total_cost)
    }
}

impl HeapWord {
    pub fn new(parent: Option<Rc<HeapWord>>, word: String, cost: u32) -> Rc<Self> {
        let total_cost = if let Some(parent) = &parent {
            Reverse(cost + parent.total_cost.0)
        } else {
            Reverse(cost)
        };
        Rc::new(Self {
            parent,
            word,
            total_cost,
        })
    }
}
#[derive(Debug, Clone)]
pub struct PooplePath {
    pub words: Vec<String>,
}

/// The poople dictionary format is as such:
/// wrd1;
/// wrd2;
/// wrd3;
/// ...
/// words are seperated by semicolons, whitespace is ignored.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoopleGraph {
    pub adjacency: HashMap<usize, Vec<usize>>,
    pub word_indices: HashMap<String, usize>,
    pub words: Vec<String>,
}

impl PoopleGraph {
    pub fn new(buf: &str) -> Self {
        let trimmed = buf
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        // Seperate and collect unique words of length 4
        let seperated = trimmed
            .split(';')
            .filter(|w| w.len() == 4)
            .collect::<HashSet<&str>>();
        let mut adjacency = HashMap::new();
        let mut word_indices = HashMap::new();
        let mut words = vec![];
        for (i, lhs_word) in seperated.iter().enumerate() {
            let mut word = lhs_word.to_string();
            words.push(word.clone());
            word_indices.insert(word, i);
            let adj_vec = seperated
                .iter()
                .cloned()
                .enumerate()
                .filter_map(|(i, w)| {
                    if hamming_metric(lhs_word, w) == 1 {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            adjacency.insert(i, adj_vec);
        }
        Self {
            adjacency,
            word_indices,
            words,
        }
    }
    pub fn adjacent_words(&self, word: &str) -> Option<impl Iterator<Item = &str>> {
        let idx = self.word_indices.get(word)?;
        let adj = self.adjacency.get(idx)?;
        Some(adj.iter().map(|i| self.words[*i].as_str()))
    }
    pub fn find_path(&self, from: &str, to: &str) -> Option<PooplePath> {
        let from = from.to_uppercase();
        let to = to.to_uppercase();
        let path = self.seek_path(&from, &to)?;
        let mut current = Some(path.clone());
        let mut words = vec![];
        while let Some(next) = current {
            words.push(next.word.clone());
            current = next.parent.clone();
        }
        Some(PooplePath { words })
    }
    fn seek_path(&self, from: &str, to: &str) -> Option<Rc<HeapWord>> {
        let mut nodes: BinaryHeap<Rc<HeapWord>> = BinaryHeap::new();
        nodes.push(HeapWord::new(None, from.to_string(), 0));
        const UPPER: usize = 600;
        let mut step = 0;
        while let Some(top) = nodes.pop() {
            if UPPER < step {
                return None;
            }
            if top.word.as_str() == to {
                return Some(top);
            }
            let adjacent = if let Some(adjacent) = self.adjacent_words(&top.word) {
                adjacent
            } else {
                continue;
            };
            for adj in adjacent {
                let node = HeapWord::new(
                    Some(top.clone()),
                    adj.to_string(),
                    hamming_metric(adj, to) as u32 + 5,
                );
                nodes.push(node);
            }
            step += 1;
        }
        None
    }
}