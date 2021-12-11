use advent2021::map::{Graph, Oopsie, Point, Points};
use itertools::Itertools;
use std::fmt;

const TEST_DATA: &str = include_str!("../../data/day9t.txt");
const DAY9_DATA: &str = include_str!("../../data/day9a.txt");

#[derive(Clone, PartialEq)]
struct Value(u8);

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u8> for Value {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl TryFrom<char> for Value {
    type Error = Oopsie;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Value(
            value.to_digit(10).ok_or(Oopsie::BadCharacter(value))? as u8,
        ))
    }
}

fn basin_around(graph: &Graph<Value>, x: usize, y: usize) -> Vec<Point<Value>> {
    let base = match graph.get(x, y) {
        None => return Vec::new(),
        Some(p) => p,
    };
    let mut retval = Vec::new();
    let mut queue = vec![base.clone()];

    while let Some(p) = queue.pop() {
        if *p.value != 9 {
            for x in graph.neighbors(p.x, p.y).drain(..) {
                if !retval.contains(&x) && !queue.contains(&x) {
                    queue.push(x);
                }
            }

            if !retval.contains(&p) {
                retval.push(p);
            }
        }
    }

    retval
}

struct LowPoints<'a> {
    graph: &'a Graph<Value>,
    underlying: &'a mut Points<'a, Value>,
}

impl<'a> Iterator for LowPoints<'a> {
    type Item = Point<'a, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let candidate = self.underlying.next()?;
            let neighbors = self.graph.neighbors(candidate.x, candidate.y);

            if neighbors.iter().all(|x| (*x).value.0 > candidate.value.0) {
                return Some(candidate);
            }
        }
    }
}

fn low_points<'a>(graph: &'a Graph<Value>, points: &'a mut Points<'a, Value>) -> LowPoints<'a> {
    LowPoints {
        graph,
        underlying: points,
    }
}

fn score_low_points(graph: &mut Graph<Value>) -> usize {
    let mut points = graph.points();
    let low_points = low_points(graph, &mut points);
    low_points.map(|x| x.value.0 as usize + 1).sum()
}

fn basins(graph: &Graph<Value>) -> usize {
    let mut points = graph.points();
    let low_points = low_points(graph, &mut points);
    low_points
        .map(|p| basin_around(graph, p.x, p.y))
        .sorted_by(|a, b| a.len().cmp(&b.len()).reverse())
        .take(3)
        .map(|x| x.len())
        .product()
}

#[test]
fn regression() {
    let mut test_graph: Graph<Value> = Graph::from_file_data(TEST_DATA).unwrap();
    assert_eq!(15, score_low_points(&mut test_graph));
    assert_eq!(1134, basins(&test_graph));

    let mut real_graph: Graph<Value> = Graph::from_file_data(DAY9_DATA).unwrap();
    assert_eq!(462, score_low_points(&mut real_graph));
    assert_eq!(1397760, basins(&real_graph));
}

fn day9() -> Result<(), Oopsie> {
    let mut test_graph: Graph<Value> = Graph::from_file_data(TEST_DATA)?;
    println!("Test low point sum: {}", score_low_points(&mut test_graph));
    println!("Test basin sum is: {}", basins(&test_graph));

    let mut real_graph: Graph<Value> = Graph::from_file_data(DAY9_DATA)?;
    println!("Real graph sum: {}", score_low_points(&mut real_graph));
    println!("Real basin sum is: {}", basins(&real_graph));

    Ok(())
}

fn main() {
    if let Err(e) = day9() {
        println!("Top level error: {}", e);
    }
}
