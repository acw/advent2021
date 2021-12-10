use itertools::Itertools;
use std::fmt;
use thiserror::Error;

const TEST_DATA: &str = include_str!("../../data/day9t.txt");
const DAY9_DATA: &str = include_str!("../../data/day9a.txt");

#[derive(Debug, Error)]
enum Oopsie {
    #[error("Tried to parse an empty graph?")]
    EmptyGraph,
    #[error("Got a weird, inconsistent line width around {0}")]
    InconsistentGraphWidth(usize),
    #[error("Got weird character parsing graph: {0}")]
    BadCharacter(char),
}

struct Graph<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

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

impl<T: Clone + TryFrom<char, Error = Oopsie>> Graph<T> {
    fn from_file_data(file_data: &str) -> Result<Graph<T>, Oopsie> {
        let mut data = Vec::with_capacity(file_data.len());
        let mut width = None;
        let mut height = 0;
        let mut temp_width = 0;

        for c in file_data.chars() {
            if c == '\n' {
                height += 1;

                if let Some(x) = width {
                    if x != temp_width {
                        return Err(Oopsie::InconsistentGraphWidth(height));
                    }
                } else {
                    width = Some(temp_width);
                }

                temp_width = 0;
            } else {
                data.push(T::try_from(c)?);
                temp_width += 1;
            }
        }

        if temp_width != 0 {
            height += 1;
        }

        if height == 0 {
            return Err(Oopsie::EmptyGraph);
        }

        Ok(Graph {
            data,
            width: width.unwrap(),
            height,
        })
    }

    fn get(&self, x: usize, y: usize) -> Option<Point<T>> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(Point {
            x,
            y,
            value: &self.data[(y * self.width) + x],
        })
    }

    fn points(&self) -> Points<'_, T> {
        Points {
            graph: self,
            curx: 0,
            cury: 0,
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<Point<T>> {
        let mut retval = Vec::new();

        if x > 0 {
            retval.push(self.get(x - 1, y).unwrap());
        }
        if y > 0 {
            retval.push(self.get(x, y - 1).unwrap());
        }
        if let Some(v) = self.get(x + 1, y) {
            retval.push(v);
        }
        if let Some(v) = self.get(x, y + 1) {
            retval.push(v);
        }

        retval
    }
}

impl Graph<Value> {
    fn basin_around(&self, x: usize, y: usize) -> Vec<Point<Value>> {
        let base = match self.get(x, y) {
            None => return Vec::new(),
            Some(p) => p,
        };
        let mut retval = Vec::new();
        let mut queue = vec![base.clone()];

        while let Some(p) = queue.pop() {
            if *p.value != 9 {
                for x in self.neighbors(p.x, p.y).drain(..) {
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
}

#[derive(Clone)]
struct Point<'a, T> {
    x: usize,
    y: usize,
    value: &'a T,
}

impl<'a, T> fmt::Debug for Point<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<'a, T> PartialEq for Point<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Points<'a, T> {
    graph: &'a Graph<T>,
    curx: usize,
    cury: usize,
}

impl<'a, T> Iterator for Points<'a, T>
where
    T: Clone + TryFrom<char, Error = Oopsie>,
{
    type Item = Point<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cury == self.graph.height {
            return None;
        }

        let next_value = self.graph.get(self.curx, self.cury)?;

        self.curx += 1;
        if self.curx == self.graph.width {
            self.curx = 0;
            self.cury += 1;
        }

        Some(next_value)
    }
}

struct LowPoints<'a> {
    underlying: &'a mut Points<'a, Value>,
}

impl<'a> Iterator for LowPoints<'a> {
    type Item = Point<'a, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let candidate = self.underlying.next()?;
            let neighbors = self.underlying.graph.neighbors(candidate.x, candidate.y);

            if neighbors.iter().all(|x| (*x).value.0 > candidate.value.0) {
                return Some(candidate);
            }
        }
    }
}

impl<'a> Points<'a, Value> {
    fn low_points(&'a mut self) -> LowPoints<'a> {
        LowPoints { underlying: self }
    }
}

fn low_points(graph: &Graph<Value>) -> usize {
    graph
        .points()
        .low_points()
        .map(|x| x.value.0 as usize + 1)
        .sum()
}

fn basins(graph: &Graph<Value>) -> usize {
    graph
        .points()
        .low_points()
        .map(|p| graph.basin_around(p.x, p.y))
        .sorted_by(|a, b| a.len().cmp(&b.len()).reverse())
        .take(3)
        .map(|x| x.len())
        .product()
}

fn day9() -> Result<(), Oopsie> {
    let test_graph: Graph<Value> = Graph::from_file_data(TEST_DATA)?;
    println!("Test low point sum: {}", low_points(&test_graph));
    println!("Test basin sum is: {}", basins(&test_graph));

    let real_graph: Graph<Value> = Graph::from_file_data(DAY9_DATA)?;
    println!("Real graph sum: {}", low_points(&real_graph));
    println!("Real basin sum is: {}", basins(&real_graph));

    Ok(())
}

fn main() {
    if let Err(e) = day9() {
        println!("Top level error: {}", e);
    }
}
