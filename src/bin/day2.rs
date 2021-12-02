use std::num;
use std::str::FromStr;
use thiserror::Error;

#[cfg(test)]
const TEST_DATA: &str = include_str!("../../data/day2_test.txt");
const DAY2A: &str = include_str!("../../data/day2a.txt");

#[derive(Debug, Error, PartialEq)]
enum Oopsie {
    #[error("Couldn't understand '{0}' as a command.")]
    CouldntParseCommand(String),
    #[error("Couldn't parse number: {0}")]
    CouldntParseNumber(#[from] num::ParseIntError),
}

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl FromStr for Command {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix("forward ") {
            Ok(Command::Forward(isize::from_str(rest)?))
        } else if let Some(rest) = s.strip_prefix("down ") {
            Ok(Command::Down(isize::from_str(rest)?))
        } else if let Some(rest) = s.strip_prefix("up ") {
            Ok(Command::Up(isize::from_str(rest)?))
        } else {
            Err(Oopsie::CouldntParseCommand(s.to_string()))
        }
    }
}

fn from_file_data<T: FromStr>(filedata: &str) -> Result<Vec<T>, T::Err> {
    let mut retval = Vec::new();

    for line in filedata.lines() {
        retval.push(T::from_str(line)?);
    }

    Ok(retval)
}

#[test]
fn command_parsing() {
    assert_eq!(Ok(Command::Forward(5)), Command::from_str("forward 5"));
    assert_eq!(Ok(Command::Down(5)), Command::from_str("down 5"));
    assert_eq!(Ok(Command::Forward(8)), Command::from_str("forward 8"));
    assert_eq!(Ok(Command::Up(3)), Command::from_str("up 3"));
    assert_eq!(Ok(Command::Down(8)), Command::from_str("down 8"));
    assert_eq!(Ok(Command::Forward(2)), Command::from_str("forward 2"));
}

#[test]
fn file_parsing() {
    let result: Result<Vec<Command>, Oopsie> = from_file_data(TEST_DATA);
    assert!(result.is_ok());
}

#[derive(Debug)]
struct Submarine {
    interpretation: Interpretation,
    depth: isize,
    position_x: isize,
    aim: isize,
}

#[derive(Debug)]
enum Interpretation {
    Basic,
    WithAim,
}

impl Submarine {
    fn new(interpretation: Interpretation) -> Submarine {
        Submarine {
            interpretation,
            depth: 0,
            position_x: 0,
            aim: 0,
        }
    }

    fn command(&mut self, cmd: &Command) {
        match self.interpretation {
            Interpretation::Basic => match cmd {
                Command::Forward(v) => self.position_x += v,
                Command::Up(v) => self.depth -= v,
                Command::Down(v) => self.depth += v,
            },

            Interpretation::WithAim => match cmd {
                Command::Forward(v) => {
                    self.position_x += v;
                    self.depth += self.aim * v;
                }
                Command::Up(v) => self.aim -= v,
                Command::Down(v) => self.aim += v,
            },
        }
    }

    fn run(&mut self, cmds: &[Command]) {
        for cmd in cmds.iter() {
            self.command(cmd);
        }
    }

    fn distance(&self) -> isize {
        self.depth * self.position_x
    }
}

#[test]
fn basic_test_intepretation() {
    let test_commands = from_file_data(TEST_DATA).unwrap();
    let mut test_submarine = Submarine::new(Interpretation::Basic);
    test_submarine.run(&test_commands);
    assert_eq!(150, test_submarine.distance());
}

#[test]
fn test_intepretation_with_aim() {
    let test_commands = from_file_data(TEST_DATA).unwrap();
    let mut test_submarine = Submarine::new(Interpretation::WithAim);
    test_submarine.run(&test_commands);
    assert_eq!(900, test_submarine.distance());
}

// just in case I decide to be fancy and pull some stuff out of this for a
// later challenge
#[test]
fn regression_tests() {
    let commands = from_file_data(DAY2A).unwrap();

    let mut part1_submarine = Submarine::new(Interpretation::Basic);
    part1_submarine.run(&commands);
    assert_eq!(2039912, part1_submarine.distance());

    let mut part2_submarine = Submarine::new(Interpretation::WithAim);
    part2_submarine.run(&commands);
    assert_eq!(1942068080, part2_submarine.distance());
}

fn day2() -> Result<(), Oopsie> {
    let commands = from_file_data(DAY2A)?;

    let mut part1_submarine = Submarine::new(Interpretation::Basic);
    part1_submarine.run(&commands);
    println!("Part 1 distance: {}", part1_submarine.distance());

    let mut part2_submarine = Submarine::new(Interpretation::WithAim);
    part2_submarine.run(&commands);
    println!("Part 2 distance: {}", part2_submarine.distance());

    Ok(())
}

fn main() {
    if let Err(e) = day2() {
        println!("Whoops: Top-level error: {}", e);
    }
}
