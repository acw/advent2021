use advent2021::from_file_data;
use core::cmp;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[cfg(test)]
const TEST_DATA: &str = include_str!("../../data/day5t.txt");
const DAY5_DATA: &str = include_str!("../../data/day5a.txt");

#[derive(Debug, Error, PartialEq)]
enum Oopsie {
    #[error("Couldn't parse number: {0}")]
    BadNumber(#[from] ParseIntError),
    #[error("Couldn't parse a point")]
    InvalidPoint,
    #[error("Couldn't parse a segment")]
    InvalidSegment,
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            None => Err(Oopsie::InvalidPoint),
            Some((xstr, ystr)) => {
                let x = usize::from_str(xstr)?;
                let y = usize::from_str(ystr)?;

                Ok(Point { x, y })
            }
        }
    }
}

#[test]
fn basic_point_parsing() {
    assert_eq!(Ok(Point { x: 0, y: 9 }), Point::from_str("0,9"));
    assert_eq!(Ok(Point { x: 8, y: 0 }), Point::from_str("8,0"));
    assert_eq!(Ok(Point { x: 9, y: 4 }), Point::from_str("9,4"));
    assert_eq!(Ok(Point { x: 2, y: 2 }), Point::from_str("2,2"));
}

#[derive(Clone, Debug, PartialEq)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl FromStr for LineSegment {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" -> ") {
            None => Err(Oopsie::InvalidSegment),
            Some((startstr, endstr)) => {
                let start = Point::from_str(startstr)?;
                let end = Point::from_str(endstr)?;

                Ok(LineSegment { start, end })
            }
        }
    }
}

#[test]
fn basic_segment_parsing() {
    assert_eq!(
        Ok(LineSegment {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 9 }
        }),
        LineSegment::from_str("0,9 -> 5,9")
    );
    assert_eq!(
        Ok(LineSegment {
            start: Point { x: 8, y: 0 },
            end: Point { x: 0, y: 8 }
        }),
        LineSegment::from_str("8,0 -> 0,8")
    );
}

impl LineSegment {
    fn max_x(&self) -> usize {
        cmp::max(self.start.x, self.end.x)
    }

    fn max_y(&self) -> usize {
        cmp::max(self.start.y, self.end.y)
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_diagonal(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }

    fn slope(&self) -> (isize, isize) {
        (
            if self.start.x < self.end.x { 1 } else { -1 },
            if self.start.y < self.end.y { 1 } else { -1 },
        )
    }
}

#[allow(dead_code)]
struct Board {
    data: Vec<u8>,
    segments: Vec<LineSegment>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            data: vec![0; width * height],
            segments: vec![],
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[(y * self.width) + x]
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.data[(y * self.width) + x] = value;
    }

    fn add_segment(&mut self, segment: LineSegment) {
        if segment.is_vertical() {
            let mut low = cmp::min(segment.start.y, segment.end.y);
            let high = cmp::max(segment.start.y, segment.end.y);

            while low <= high {
                let current = self.get(segment.start.x, low);
                self.set(segment.start.x, low, current + 1);
                low += 1;
            }
        }

        if segment.is_horizontal() {
            let mut low = cmp::min(segment.start.x, segment.end.x);
            let high = cmp::max(segment.start.x, segment.end.x);

            while low <= high {
                let current = self.get(low, segment.start.y);
                self.set(low, segment.start.y, current + 1);
                low += 1;
            }
        }

        self.segments.push(segment);
    }

    fn add_diagonals(&mut self) {
        let diagonals: Vec<LineSegment> = self
            .segments
            .iter()
            .filter(|x| x.is_diagonal())
            .cloned()
            .collect();

        for segment in diagonals.iter() {
            if !segment.is_horizontal() && !segment.is_vertical() {
                let (delta_x, delta_y) = segment.slope();
                let mut x = segment.start.x as isize;
                let mut y = segment.start.y as isize;
                let target_x = segment.end.x as isize;
                let target_y = segment.end.y as isize;

                loop {
                    let current = self.get(x as usize, y as usize);
                    self.set(x as usize, y as usize, current + 1);

                    if x == target_x && y == target_y {
                        break;
                    }

                    x += delta_x;
                    y += delta_y;
                }
            }
        }
    }

    fn _print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{:02} ", self.get(x, y));
            }
            println!();
        }
    }

    fn cross_overs(&self) -> usize {
        self.data.iter().filter(|x| **x >= 2).count()
    }
}

impl From<Vec<LineSegment>> for Board {
    fn from(mut inputs: Vec<LineSegment>) -> Self {
        let width = inputs.iter().map(|x| x.max_x()).max().unwrap();
        let height = inputs.iter().map(|x| x.max_y()).max().unwrap();
        let mut board = Board::new(width + 1, height + 1);

        for segment in inputs.drain(..) {
            board.add_segment(segment);
        }

        board
    }
}

impl FromStr for Board {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = from_file_data(s)?;
        Ok(Board::from(segments))
    }
}

#[test]
fn regression() {
    let mut test_board = Board::from_str(TEST_DATA).unwrap();
    assert_eq!(5, test_board.cross_overs());
    test_board.add_diagonals();
    assert_eq!(12, test_board.cross_overs());

    let mut main_board = Board::from_str(DAY5_DATA).unwrap();
    assert_eq!(7468, main_board.cross_overs());
    main_board.add_diagonals();
    assert_eq!(22364, main_board.cross_overs());
}

fn day5() -> Result<(), Oopsie> {
    let mut main_board = Board::from_str(DAY5_DATA)?;

    println!(
        "Board has {} cross-overs without diagonals.",
        main_board.cross_overs()
    );
    main_board.add_diagonals();
    println!(
        "Board has {} cross-overs with diagonals.",
        main_board.cross_overs()
    );

    Ok(())
}

fn main() {
    if let Err(e) = day5() {
        println!("Top-level error: {}", e);
    }
}
