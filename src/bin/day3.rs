use advent2021::from_file_data;
use std::fmt::{self, Write};
use std::str::FromStr;
use thiserror::Error;

#[cfg(test)]
const TEST_DATA: &str = include_str!("../../data/day3t.txt");
const DAY3A_DATA: &str = include_str!("../../data/day3a.txt");

#[derive(Debug, Error, PartialEq)]
enum Oopsie {
    #[error("Invalid bit found in number: {0}")]
    InvalidBit(char),
    #[error("Indeterminate value in filter; ran off the end")]
    IndeterminateValue,
}

#[derive(Clone, Debug, PartialEq)]
struct Datum(Vec<bool>);

impl FromStr for Datum {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut retval = Vec::with_capacity(s.len());

        for c in s.chars() {
            match c {
                '0' => retval.push(false),
                '1' => retval.push(true),
                bad => return Err(Oopsie::InvalidBit(bad)),
            }
        }

        Ok(Datum(retval))
    }
}

impl fmt::Display for Datum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in self.0.iter() {
            if *v {
                f.write_char('1')?
            } else {
                f.write_char('0')?
            }
        }
        Ok(())
    }
}

impl From<Datum> for usize {
    fn from(x: Datum) -> Self {
        let mut retval = 0;

        for b in x.0.iter() {
            retval <<= 1;
            if *b {
                retval += 1;
            }
        }

        retval
    }
}

#[test]
fn can_read_diagnostic_data_bits() {
    assert_eq!(
        Ok(Datum(vec![false, false, true, false, false])),
        Datum::from_str("00100")
    );
    assert_eq!(
        Ok(Datum(vec![true, true, true, true, false])),
        Datum::from_str("11110")
    );
}

#[derive(Debug, PartialEq)]
struct Diagnostics(Vec<Datum>);

impl<'a> From<&'a [Datum]> for Diagnostics {
    fn from(data: &'a [Datum]) -> Self {
        Diagnostics(data.to_vec())
    }
}

impl FromStr for Diagnostics {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Diagnostics(from_file_data(s)?))
    }
}

impl Diagnostics {
    fn most_common_bits(&self) -> Vec<bool> {
        let total_items = self.0.len();
        let entries_per_datum = self.0.iter().map(|x| x.0.len()).min().unwrap();
        let initial_values = vec![0; entries_per_datum];
        let true_counts = self.0.iter().fold(initial_values, |acc, x| {
            acc.iter()
                .zip(x.0.iter())
                .map(|(count, val)| if *val { count + 1 } else { *count })
                .collect()
        });
        true_counts
            .iter()
            .map(|count| *count >= ((total_items + 1) / 2))
            .collect()
    }

    fn get_rates(&self) -> (usize, usize) {
        self.most_common_bits()
            .iter()
            .fold((0, 0), |(gacc, eacc), most_common_is_one| {
                let mut new_gamma = gacc << 1;
                let mut new_epsilon = eacc << 1;

                if *most_common_is_one {
                    new_gamma += 1;
                } else {
                    new_epsilon += 1;
                }

                (new_gamma, new_epsilon)
            })
    }

    fn rating_generator<F: Fn(bool, bool) -> bool>(&self, f: F) -> Result<usize, Oopsie> {
        let mut candidates = self.0.clone();
        let mut offset = 0;

        while candidates.len() > 1 {
            let most_commons = Diagnostics::from(&candidates[..]).most_common_bits();
            let mut die_horribly = false;

            candidates.retain(|datum| match datum.0.get(offset) {
                None => {
                    die_horribly = true;
                    false
                }
                Some(current_bit) => match most_commons.get(offset) {
                    None => {
                        die_horribly = true;
                        false
                    }
                    Some(most_common_bit) => f(*current_bit, *most_common_bit),
                },
            });

            if die_horribly {
                return Err(Oopsie::IndeterminateValue);
            }

            offset += 1;
        }

        match candidates.pop() {
            None => Err(Oopsie::IndeterminateValue),
            Some(x) => Ok(usize::from(x)),
        }
    }

    fn o2_generator_rating(&self) -> Result<usize, Oopsie> {
        self.rating_generator(|current, most_common| current == most_common)
    }

    fn co2_scrubber_rating(&self) -> Result<usize, Oopsie> {
        self.rating_generator(|current, most_common| current != most_common)
    }
}

#[test]
fn part1_example_works() {
    let diags = Diagnostics::from_str(TEST_DATA).unwrap();
    let (gamma_rate, epsilon_rate) = diags.get_rates();
    assert_eq!(22, gamma_rate);
    assert_eq!(9, epsilon_rate);
}

#[test]
fn part2_example_works() {
    let diags = Diagnostics::from_str(TEST_DATA).unwrap();
    let o2r = diags.o2_generator_rating();
    let co2r = diags.co2_scrubber_rating();
    assert_eq!(Ok(23), o2r);
    assert_eq!(Ok(10), co2r);
}

#[test]
fn regression_tests() {
    let diagnostics = Diagnostics::from_str(DAY3A_DATA).unwrap();
    let (p1gamma, p1epsilon) = diagnostics.get_rates();
    assert_eq!(1491, p1gamma);
    assert_eq!(2604, p1epsilon);
    assert_eq!(Ok(1305), diagnostics.o2_generator_rating());
    assert_eq!(Ok(2594), diagnostics.co2_scrubber_rating());
}

fn day3() -> Result<(), Oopsie> {
    let diagnostics = Diagnostics::from_str(DAY3A_DATA).unwrap();

    let (gamma_rate, epsilon_rate) = diagnostics.get_rates();
    println!(
        "For part #1, computed gamma rate {}, epsilon rate {}, for a power consumption {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    let o2_generator_rating = diagnostics.o2_generator_rating()?;
    let co2_scrubber_rating = diagnostics.co2_scrubber_rating()?;
    println!(
        "For part #2, computed O2 generator rating is {}, CO2 scrubber rating is {}, for a life support rating of {}",
        o2_generator_rating,
        co2_scrubber_rating,
        o2_generator_rating * co2_scrubber_rating
    );
    Ok(())
}

fn main() {
    if let Err(e) = day3() {
        println!("Whoops: Top-level error: {}", e);
    }
}
