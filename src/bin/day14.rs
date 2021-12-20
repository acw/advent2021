use std::collections::HashMap;
use thiserror::Error;

const TEST_DATA: &str = include_str!("../../data/day14t.txt");
const REAL_DATA: &str = include_str!("../../data/day14a.txt");

#[derive(Debug, Error, PartialEq)]
enum Oopsie {
    #[error("Bad transform declaration: {0}")]
    BadTransform(String),
    #[error("No data found?!")]
    NoData,
}

struct Transform {
    lead: char,
    follow: char,
    inject: char,
}

impl TryFrom<&str> for Transform {
    type Error = Oopsie;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.split_once(" -> ") {
            None => Err(Oopsie::BadTransform(s.to_string())),
            Some((from, to_str)) => Ok(Transform {
                lead: from.chars().next().unwrap(),
                follow: from.chars().nth(1).unwrap(),
                inject: to_str.chars().next().unwrap(),
            }),
        }
    }
}

fn read_file(contents: &str) -> Result<(String, Vec<Transform>), Oopsie> {
    let mut lines = contents.lines();
    let mut transforms = Vec::new();

    let base_str = match lines.next() {
        None => return Err(Oopsie::NoData),
        Some(x) => x.to_string(),
    };

    for line in lines {
        if line.is_empty() {
            continue;
        }

        transforms.push(Transform::try_from(line)?);
    }

    Ok((base_str, transforms))
}

type Pair = (char, char);
type TransformDictionary = HashMap<Pair, [Pair; 2]>;

fn build_transform_dictionary(mut transforms: Vec<Transform>) -> TransformDictionary {
    let mut result = HashMap::with_capacity(transforms.len());

    for transform in transforms.drain(..) {
        result.insert(
            (transform.lead, transform.follow),
            [
                (transform.lead, transform.inject),
                (transform.inject, transform.follow),
            ],
        );
    }

    result
}

type State = HashMap<Pair, usize>;

macro_rules! insert_update {
    ($dict: expr, $key: expr, $value: expr, |$x: ident| $body: expr) => {
        match $dict.get_mut(&$key) {
            None => {
                $dict.insert($key, $value);
            }
            Some($x) => *$x = $body,
        }
    };
}

fn build_initial_state(base: &str) -> State {
    let mut state = HashMap::new();
    let mut chars = base.chars().peekable();

    while let Some(c1) = chars.next() {
        match chars.peek() {
            None => break,
            Some(c2) => insert_update!(state, (c1, *c2), 1, |v| *v + 1),
        }
    }

    state
}

fn step(base: &State, transforms: &TransformDictionary) -> State {
    let mut result = HashMap::new();

    for (pair, count) in base.iter() {
        match transforms.get(pair) {
            None => insert_update!(result, *pair, *count, |v| *v + count),
            Some([p1, p2]) => {
                insert_update!(result, *p1, *count, |v| *v + count);
                insert_update!(result, *p2, *count, |v| *v + count);
            }
        }
    }

    result
}

fn steps(base: &State, steps: usize, transforms: &TransformDictionary) -> State {
    let mut result = base.clone();

    for i in 0..steps {
        println!("At step {}, length is {}", i, result.len());
        result = step(&result, transforms);
    }

    result
}

fn counts(polymer: &State) -> HashMap<char, usize> {
    let mut char_counts = HashMap::new();

    for ((c1, c2), count) in polymer.iter() {
        insert_update!(char_counts, *c1, *count, |v| *v + count);
        insert_update!(char_counts, *c2, *count, |v| *v + count);
    }

    char_counts
}

fn score(map: &HashMap<char, usize>) -> usize {
    let high = map.values().max().unwrap();
    let low = map.values().min().unwrap();

    (high - low) / 2
}

fn main() -> Result<(), Oopsie> {
    let (test_base, test_transforms) = read_file(TEST_DATA)?;
    let (real_base, real_transforms) = read_file(REAL_DATA)?;
    let test_initial = build_initial_state(&test_base);
    let real_initial = build_initial_state(&real_base);
    let test_transform_dict = build_transform_dictionary(test_transforms);
    let real_transform_dict = build_transform_dictionary(real_transforms);

    println!("test base: {:?}", test_initial);
    println!("one step: {:?}", step(&test_initial, &test_transform_dict));
    println!(
        "four steps: {:?}",
        steps(&test_initial, 4, &test_transform_dict)
    );
    println!(
        "Length at 10 steps: {:?}",
        steps(&test_initial, 10, &test_transform_dict).len()
    );
    println!(
        "Counts at 10 steps: {:?}",
        counts(&steps(&test_initial, 10, &test_transform_dict))
    );
    println!(
        "Score at 10 steps: {:?}",
        score(&counts(&steps(&test_initial, 10, &test_transform_dict)))
    );

    println!(
        "Real score at 10 steps: {:?}",
        score(&counts(&steps(&real_initial, 10, &real_transform_dict)))
    );
    println!(
        "Real score at 40 steps: {:?}",
        score(&counts(&steps(&real_initial, 40, &real_transform_dict)))
    );
    Ok(())
}
