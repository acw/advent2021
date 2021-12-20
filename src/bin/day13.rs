use core::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

const TEST_DATA: &str = include_str!("../../data/day13t.txt");
const REAL_DATA: &str = include_str!("../../data/day13a.txt");

#[derive(Debug, Error, PartialEq)]
enum Oopsie {
    #[error("Couldn't parse point based on this string: {0}")]
    InvalidPoint(String),
    #[error("Couldn't parse fold based on this string: {0}")]
    BadFold(String),
    #[error("Couldn't parse number: {0}")]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            None => Err(Oopsie::InvalidPoint(s.to_string())),
            Some((xstr, ystr)) => {
                let x = usize::from_str(xstr)?;
                let y = usize::from_str(ystr)?;
                Ok(Point { x, y })
            }
        }
    }
}

enum Fold {
    AlongX(usize),
    AlongY(usize),
}

impl FromStr for Fold {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix("fold along ") {
            None => Err(Oopsie::BadFold(s.to_string())),
            Some(line) => {
                if let Some(num) = line.strip_prefix("x=") {
                    return Ok(Fold::AlongX(usize::from_str(num)?));
                }

                if let Some(num) = line.strip_prefix("y=") {
                    return Ok(Fold::AlongY(usize::from_str(num)?));
                }

                Err(Oopsie::BadFold(s.to_string()))
            }
        }
    }
}

fn parse_file(contents: &str) -> Result<(Vec<Point>, Vec<Fold>), Oopsie> {
    let mut reading_points = true;
    let mut points = Vec::new();
    let mut folds = Vec::new();

    for line in contents.lines() {
        if line.is_empty() {
            reading_points = false;
            continue;
        }

        if reading_points {
            points.push(Point::from_str(line)?);
        } else {
            folds.push(Fold::from_str(line)?);
        }
    }

    Ok((points, folds))
}

impl Point {
    fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::AlongY(value) => {
                if &self.y > value {
                    let difference = self.y - value;
                    self.y = value - difference;
                }
            }
            Fold::AlongX(value) => {
                if &self.x > value {
                    let difference = self.x - value;
                    self.x = value - difference;
                }
            }
        }
    }
}

fn fold(vec: &mut Vec<Point>, fold: &Fold) {
    for point in vec.iter_mut() {
        point.fold(fold);
    }

    vec.sort_unstable();
    vec.dedup();
}

fn main() -> Result<(), Oopsie> {
    let (mut test_points, test_folds) = parse_file(TEST_DATA)?;

    fold(&mut test_points, &test_folds[0]);
    println!("{} test points: {:?}", test_points.len(), test_points);
    fold(&mut test_points, &test_folds[1]);
    println!("{} test points: {:?}", test_points.len(), test_points);
    let (mut real_points, real_folds) = parse_file(REAL_DATA)?;
    fold(&mut real_points, &real_folds[0]);
    println!("{} real points after first fold", real_points.len());
    for instr in real_folds[1..].iter() {
        fold(&mut real_points, instr);
    }
    let max_x = real_points.iter().map(|x| x.x).max().unwrap();
    let max_y = real_points.iter().map(|x| x.y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if real_points.contains(&Point { x, y }) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!()
    }

    Ok(())
}
