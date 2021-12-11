use advent2021::map::{Graph, Oopsie};
use core::fmt;

#[cfg(test)]
const TEST_DATA: &str = include_str!("../../data/day11t.txt");
const REAL_DATA: &str = include_str!("../../data/day11a.txt");

pub struct Level(u16);

impl fmt::Debug for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u16> for Level {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl TryFrom<char> for Level {
    type Error = Oopsie;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Level(
            value.to_digit(10).ok_or(Oopsie::BadCharacter(value))? as u16,
        ))
    }
}

impl Level {
    fn increment(&mut self) -> bool {
        if self.0 == 9 {
            self.0 = 0;
            true
        } else {
            self.0 += 1;
            false
        }
    }
}

fn step(map: &mut Graph<Level>) -> usize {
    let mut flash_points: Vec<(usize, usize)> = map
        .points_mut()
        .map(|p| (p.x, p.y, p.value.increment()))
        .filter_map(|(x, y, flashed)| if flashed { Some((x, y)) } else { None })
        .collect();
    let mut queue = flash_points.clone();

    // loop until we have flashed everyone
    while let Some((x, y)) = queue.pop() {
        for (nx, ny) in map.neighbor_points(x, y).drain(..) {
            if !flash_points.contains(&(nx, ny)) && map.get_mut(nx, ny).unwrap().value.increment() {
                flash_points.push((nx, ny));
                queue.push((nx, ny));
            }
        }
    }

    flash_points.len()
}

fn run(map: &mut Graph<Level>, steps: usize) -> usize {
    let mut total_flashes = 0;

    for _ in 0..steps {
        total_flashes += step(map);
    }

    total_flashes
}

fn full_flash_step(map: &mut Graph<Level>) -> usize {
    let mut count = 0;

    loop {
        let flash_count = step(map);

        count += 1;
        if flash_count == map.size() {
            return count;
        }
    }
}

#[test]
fn regression() {
    let mut test_data_steps: Graph<Level> = Graph::from_file_data(TEST_DATA).unwrap();
    assert_eq!(0, step(&mut test_data_steps));
    assert_eq!(35, step(&mut test_data_steps));
    assert_eq!(45, step(&mut test_data_steps));
    assert_eq!(16, step(&mut test_data_steps));
    assert_eq!(8, step(&mut test_data_steps));
    assert_eq!(1, step(&mut test_data_steps));
    assert_eq!(7, step(&mut test_data_steps));
    assert_eq!(24, step(&mut test_data_steps));
    assert_eq!(39, step(&mut test_data_steps));
    assert_eq!(29, step(&mut test_data_steps));

    let mut test_data10: Graph<Level> = Graph::from_file_data(TEST_DATA).unwrap();
    assert_eq!(204, run(&mut test_data10, 10));

    let mut test_data: Graph<Level> = Graph::from_file_data(TEST_DATA).unwrap();
    assert_eq!(1656, run(&mut test_data, 100));

    let mut test_data_wait: Graph<Level> = Graph::from_file_data(TEST_DATA).unwrap();
    assert_eq!(195, full_flash_step(&mut test_data_wait));
}

fn day11() -> Result<(), Oopsie> {
    let mut real_data: Graph<Level> = Graph::from_file_data(REAL_DATA)?;
    println!("Flashes after 100 steps: {}", run(&mut real_data, 100));

    let mut real_data_wait = Graph::from_file_data(REAL_DATA)?;
    println!(
        "{} steps until they all flash.",
        full_flash_step(&mut real_data_wait)
    );

    Ok(())
}

fn main() {
    if let Err(e) = day11() {
        println!("Top level error: {}", e);
    }
}
