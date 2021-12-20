use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
enum Oopsie {
    #[error("Bad line found: {0}")]
    BadLine(String),
}

const TEST1_DATA: &str = include_str!("../../data/day12t1.txt");
const TEST2_DATA: &str = include_str!("../../data/day12t2.txt");
const REAL_DATA: &str = include_str!("../../data/day12a.txt");

struct Graph<'a> {
    edges: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Graph<'a> {
    fn add_edge(&mut self, from: &'a str, to: &'a str) {
        match self.edges.get_mut(from) {
            None => {
                let mut new_set = HashSet::new();
                new_set.insert(to);
                self.edges.insert(from, new_set);
            }
            Some(set) => {
                set.insert(to);
            }
        }
    }

    fn paths(&self) -> Paths<'_> {
        Paths {
            graph: self,
            state: VecDeque::from([vec!["start"]]),
        }
    }
}

struct Paths<'a> {
    graph: &'a Graph<'a>,
    state: VecDeque<Vec<&'a str>>,
}

fn count<T: PartialEq>(items: &[T], item: &T) -> usize {
    let mut num = 0;

    for x in items.iter() {
        if x == item {
            num += 1;
        }
    }

    num
}

fn still_valid(items: &[&str]) -> bool {
    let mut counts = HashMap::new();

    for item in items.iter() {
        if item.chars().all(|x| x.is_lowercase()) {
            if counts.contains_key(item) {
                counts.insert(item, 2);
            } else {
                counts.insert(item, 1);
            }
        }
    }

    let mut caught_two = false;

    for count in counts.values() {
        if caught_two && count > &1 {
            return false;
        }

        if count > &1 {
            caught_two = true;
        }
    }

    true
}

impl<'a> Iterator for Paths<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(possible) = self.state.pop_front() {
            match possible.last() {
                None => panic!("Internal error; empty path in queue"),
                Some(&"end") => return Some(possible),
                Some(v) => {
                    for x in self.graph.edges.get(v).unwrap_or(&HashSet::new()) {
                        if x == &"start" {
                            continue;
                        }

                        if !x.chars().all(|x| x.is_lowercase()) || count(&possible, x) < 2 {
                            let mut copy = possible.clone();
                            copy.push(x);
                            if still_valid(&copy) {
                                self.state.push_back(copy);
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

impl<'a> TryFrom<&'a str> for Graph<'a> {
    type Error = Oopsie;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut graph = Graph {
            edges: HashMap::new(),
        };

        for line in value.lines() {
            match line.split_once('-') {
                None => return Err(Oopsie::BadLine(line.to_string())),
                Some((left, right)) => {
                    graph.add_edge(left, right);
                    graph.add_edge(right, left);
                }
            }
        }

        Ok(graph)
    }
}

fn main() {
    let test1 = Graph::try_from(TEST1_DATA).unwrap();
    let test2 = Graph::try_from(TEST2_DATA).unwrap();
    let real = Graph::try_from(REAL_DATA).unwrap();

    println!("Test #1 count: {}", test1.paths().count());
    println!("Test #2 count: {}", test2.paths().count());
    println!("Real count: {}", real.paths().count());
}
