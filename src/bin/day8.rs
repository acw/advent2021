use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[cfg(test)]
const TEST_DATA: &str = include_str!("../../data/day8t.txt");
const DAY8_DATA: &str = include_str!("../../data/day8a.txt");

#[derive(Clone)]
struct ProblemInput<'a> {
    signal_patterns: [&'a str; 10],
    output_values: [&'a str; 4],
}

impl<'a> From<&'a str> for ProblemInput<'a> {
    fn from(x: &'a str) -> Self {
        let (signals, outputs) = x.split_once(" | ").unwrap();
        let mut signal_patterns = [""; 10];
        let mut output_values = [""; 4];

        for (i, val) in signals.split(' ').enumerate() {
            signal_patterns[i] = val;
        }

        for (i, val) in outputs.split(' ').enumerate() {
            output_values[i] = val;
        }

        ProblemInput {
            signal_patterns,
            output_values,
        }
    }
}

#[test]
fn problem_input() {
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let parsed = ProblemInput::from(input);
    assert_eq!(
        ["acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"],
        parsed.signal_patterns
    );
    assert_eq!(["cdfeb", "fcadb", "cdfeb", "cdbaf"], parsed.output_values);
}

fn from_file_data(input_file: &str) -> Vec<ProblemInput<'_>> {
    let mut retval = Vec::new();

    for line in input_file.lines() {
        retval.push(ProblemInput::from(line));
    }

    retval
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Segment {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
}

const SEGMENTS_USED: [&[Segment]; 10] = [
    /* 0 */
    &[
        Segment::Top,
        Segment::TopLeft,
        Segment::TopRight,
        Segment::BottomLeft,
        Segment::BottomRight,
        Segment::Bottom,
    ],
    /* 1 */ &[Segment::TopRight, Segment::BottomRight],
    /* 2 */
    &[
        Segment::Top,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomLeft,
        Segment::Bottom,
    ],
    /* 3 */
    &[
        Segment::Top,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomRight,
        Segment::Bottom,
    ],
    /* 4 */
    &[
        Segment::TopLeft,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomRight,
    ],
    /* 5 */
    &[
        Segment::Top,
        Segment::TopLeft,
        Segment::Middle,
        Segment::BottomRight,
        Segment::Bottom,
    ],
    /* 6 */
    &[
        Segment::Top,
        Segment::TopLeft,
        Segment::Middle,
        Segment::BottomRight,
        Segment::BottomLeft,
        Segment::Bottom,
    ],
    /* 7 */ &[Segment::Top, Segment::TopRight, Segment::BottomRight],
    /* 8 */
    &[
        Segment::Top,
        Segment::TopLeft,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomLeft,
        Segment::BottomRight,
        Segment::Bottom,
    ],
    /* 9 */
    &[
        Segment::Top,
        Segment::TopLeft,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomRight,
        Segment::Bottom,
    ],
];

fn recognize_number(segs: &[Segment]) -> Option<usize> {
    for (result, digit_info) in SEGMENTS_USED.iter().enumerate() {
        if digit_info.len() == segs.len() && segs.iter().all(|x| digit_info.contains(x)) {
            return Some(result);
        }
    }

    None
}

fn reasonable_assignments(assigns: &HashMap<char, Segment>) -> bool {
    let mut tester = HashSet::new();

    for value in assigns.values() {
        if tester.contains(value) {
            return false;
        }
        tester.insert(value);
    }

    true
}

fn build_local_iterator<'a>(
    input: (&'a char, &'a HashSet<Segment>),
) -> impl Iterator<Item = (char, Segment)> + Clone + 'a {
    let (c, set) = input;
    set.iter().map(|x| (*c, *x))
}

impl<'a> ProblemInput<'a> {
    fn unique_outputs(&self) -> usize {
        let mut result = 0;

        for v in self.output_values.iter() {
            let len = v.len();

            if len == 2 || len == 4 || len == 3 || len == 7 {
                result += 1;
            }
        }

        result
    }

    fn find_letters_for(&self, len: usize) -> &str {
        for x in self.signal_patterns {
            if x.len() == len {
                return x;
            }
        }
        panic!("Couldn't find letters for size {}", len);
    }

    fn possible_assignments(&self) -> HashMap<char, HashSet<Segment>> {
        let mut result = HashMap::new();

        for label in 'a'..='g' {
            result.insert(label, SEGMENTS_USED[8].iter().copied().collect());
        }

        let one = self.find_letters_for(2);
        let four = self.find_letters_for(4);
        let seven = self.find_letters_for(3);

        for letter in one.chars() {
            result.insert(
                letter,
                HashSet::from([Segment::TopRight, Segment::BottomRight]),
            );
        }

        for letter in four.chars() {
            if !one.contains(letter) {
                result.insert(letter, HashSet::from([Segment::TopLeft, Segment::Middle]));
            }
        }

        for letter in seven.chars() {
            if !one.contains(letter) {
                result.insert(letter, HashSet::from([Segment::Top]));
            }
        }

        result
    }

    fn assignments_validate(&self, assignments: &HashMap<char, Segment>) -> bool {
        let mut found = HashSet::new();

        for input in self.signal_patterns {
            let segments: Vec<Segment> = input
                .chars()
                .map(|x| assignments.get(&x).unwrap())
                .copied()
                .collect();
            match recognize_number(&segments) {
                None => return false,
                Some(x) if found.contains(&x) => return false,
                Some(x) => {
                    found.insert(x);
                }
            }
        }

        true
    }

    fn compute_output(&self, assignments: HashMap<char, Segment>) -> Option<usize> {
        let mut result = 0;

        for output in self.output_values.iter() {
            let segments: Vec<Segment> = output
                .chars()
                .map(|x| assignments.get(&x).unwrap())
                .copied()
                .collect();
            let value = recognize_number(&segments)?;
            result = (result * 10) + value;
        }

        Some(result)
    }

    fn solve(&self) -> Option<usize> {
        let possible_sets = self.possible_assignments();
        let possibles = possible_sets
            .iter()
            .map(build_local_iterator)
            .multi_cartesian_product()
            .map(|x| HashMap::from_iter(x.iter().cloned()))
            .filter(reasonable_assignments);

        for assignments in possibles {
            if self.assignments_validate(&assignments) {
                if let Some(result) = self.compute_output(assignments) {
                    return Some(result);
                }
            }
        }

        None
    }
}

fn count_unique_outputs(inputs: &[ProblemInput]) -> usize {
    inputs.iter().map(|x| x.unique_outputs()).sum()
}

fn sum_outputs(inputs: &[ProblemInput]) -> usize {
    inputs.iter().map(|x| x.solve().unwrap()).sum()
}

#[test]
fn regression() {
    let example_input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let example = ProblemInput::from(example_input);
    assert_eq!(5353, sum_outputs(&[example]));

    let test_data = from_file_data(TEST_DATA);
    assert_eq!(26, count_unique_outputs(&test_data));
    assert_eq!(61229, sum_outputs(&test_data));
    let real_data = from_file_data(DAY8_DATA);
    assert_eq!(519, count_unique_outputs(&real_data));
}

fn main() {
    let real_data = from_file_data(DAY8_DATA);
    println!("Part #1: {}", count_unique_outputs(&real_data));
    println!("Part #2: {}", sum_outputs(&real_data));
}
