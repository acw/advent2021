use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[cfg(test)]
const TEST_DATA: &str = include_str!("../../data/day4t.txt");
const DAY4_DATA: &str = include_str!("../../data/day4a.txt");

#[derive(Debug, Error, PartialEq)]
enum Oopsie {
    #[error("The data file for the problem was empty.")]
    EmptyFile,
    #[error("Error parsing number: {0}")]
    NumberParseError(#[from] ParseIntError),
    #[error("Invalid board size input: {0}")]
    InvalidBoardSize(usize),
    #[error("No one is a winner :(")]
    NoWinnerFound,
}

#[derive(Clone)]
struct Spot {
    number: u64,
    marked: bool,
}

impl Spot {
    fn new(number: u64) -> Spot {
        Spot {
            number,
            marked: false,
        }
    }
}

impl fmt::Display for Spot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}{}",
            self.number,
            if self.marked { '*' } else { ' ' }
        )
    }
}

#[derive(Clone)]
struct Board {
    spots: [Spot; 25],
}

impl Board {
    fn new(spot_numbers: &[u64]) -> Result<Board, Oopsie> {
        if spot_numbers.len() != 25 {
            return Err(Oopsie::InvalidBoardSize(spot_numbers.len()));
        }

        Ok(Board {
            spots: [
                Spot::new(spot_numbers[0]),
                Spot::new(spot_numbers[1]),
                Spot::new(spot_numbers[2]),
                Spot::new(spot_numbers[3]),
                Spot::new(spot_numbers[4]),
                Spot::new(spot_numbers[5]),
                Spot::new(spot_numbers[6]),
                Spot::new(spot_numbers[7]),
                Spot::new(spot_numbers[8]),
                Spot::new(spot_numbers[9]),
                Spot::new(spot_numbers[10]),
                Spot::new(spot_numbers[11]),
                Spot::new(spot_numbers[12]),
                Spot::new(spot_numbers[13]),
                Spot::new(spot_numbers[14]),
                Spot::new(spot_numbers[15]),
                Spot::new(spot_numbers[16]),
                Spot::new(spot_numbers[17]),
                Spot::new(spot_numbers[18]),
                Spot::new(spot_numbers[19]),
                Spot::new(spot_numbers[20]),
                Spot::new(spot_numbers[21]),
                Spot::new(spot_numbers[22]),
                Spot::new(spot_numbers[23]),
                Spot::new(spot_numbers[24]),
            ],
        })
    }

    fn print(&self) {
        let mut count = 0;

        for spot in self.spots.iter() {
            print!("{}", spot);

            count += 1;
            if count == 5 {
                println!();
                count = 0;
            } else {
                print!(" ");
            }
        }
    }

    fn get(&self, x: usize, y: usize) -> &Spot {
        &self.spots[(y * 5) + x]
    }

    fn won(&self) -> bool {
        for i in 0..5 {
            if self.get(i, 0).marked
                && self.get(i, 1).marked
                && self.get(i, 2).marked
                && self.get(i, 3).marked
                && self.get(i, 4).marked
            {
                return true;
            }

            if self.get(0, i).marked
                && self.get(1, i).marked
                && self.get(2, i).marked
                && self.get(3, i).marked
                && self.get(4, i).marked
            {
                return true;
            }
        }

        false
    }

    fn mark(&mut self, called: u64) {
        for spot in self.spots.iter_mut() {
            if spot.number == called {
                spot.marked = true;
            }
        }
    }

    fn unmarked(&self) -> impl Iterator<Item = u64> + '_ {
        self.spots.iter().filter(|x| !x.marked).map(|x| x.number)
    }
}

fn parse_state(fileblob: &str) -> Result<(Vec<u64>, Vec<Board>), Oopsie> {
    let mut lines = fileblob.lines();

    let calls = match lines.next() {
        None => return Err(Oopsie::EmptyFile),
        Some(x) => {
            let mut retval = Vec::new();

            for str_value in x.split(',') {
                retval.push(u64::from_str(str_value)?);
            }

            retval
        }
    };

    let mut boards = Vec::new();
    let mut workspace = Vec::new();

    for work_line in lines {
        let clean_line = work_line.trim();

        if clean_line.is_empty() {
            if !workspace.is_empty() {
                boards.push(Board::new(&workspace)?);
                workspace.clear();
            }

            continue;
        }

        for value in clean_line.split(' ') {
            if !value.is_empty() {
                workspace.push(u64::from_str(value)?);
            }
        }
    }

    if !workspace.is_empty() {
        boards.push(Board::new(&workspace)?);
    }

    Ok((calls, boards))
}

fn play(calls: &[u64], input_boards: &[Board]) -> Result<(u64, u64), Oopsie> {
    let mut first_won_value = None;
    let mut boards = input_boards.to_vec();

    for call in calls.iter() {
        println!("\nCalling {}", call);

        // mark all the boards
        for board in boards.iter_mut() {
            board.mark(*call);

            println!();
            board.print();
        }

        if first_won_value.is_none() {
            // did anyone win?
            for board in boards.iter() {
                if board.won() {
                    let unmarked_sum = board.unmarked().sum::<u64>();
                    first_won_value = Some(call * unmarked_sum);
                }
            }
        }

        // are we in our final case?
        if boards.iter().all(|x| x.won()) && boards.len() == 1 {
            let unmarked_sum = boards[0].unmarked().sum::<u64>();
            return Ok((first_won_value.unwrap(), unmarked_sum * call));
        }

        boards.retain(|x| !x.won());
    }

    Err(Oopsie::NoWinnerFound)
}

#[test]
fn regression() {
    let (test_calls, test_boards) = parse_state(TEST_DATA).unwrap();
    let (first_won_res, last_won_res) = play(&test_calls, &test_boards).unwrap();
    assert_eq!(4512, first_won_res);
    assert_eq!(1924, last_won_res);

    let (data_calls, data_boards) = parse_state(DAY4_DATA).unwrap();
    let (first_won_res, last_won_res) = play(&data_calls, &data_boards).unwrap();
    assert_eq!(31424, first_won_res);
    assert_eq!(23042, last_won_res);
}

fn day4() -> Result<(), Oopsie> {
    let (calls, boards) = parse_state(DAY4_DATA).unwrap();
    let (first_won_res, last_won_res) = play(&calls, &boards)?;
    println!(
        "Part #1 result is {}, part #2 result is {}",
        first_won_res, last_won_res
    );
    Ok(())
}

fn main() {
    if let Err(e) = day4() {
        println!("Top-level error: {}", e);
    }
}
