use std::cmp::{max, Ordering};
use std::ops::RangeInclusive;

struct TargetArea {
    target_x: RangeInclusive<isize>,
    target_y: RangeInclusive<isize>,
}

impl TargetArea {
    fn contains(&self, x: isize, y: isize) -> bool {
        self.target_x.contains(&x) && self.target_y.contains(&y)
    }
}

struct Probe {
    pos_x: isize,
    pos_y: isize,
    vel_x: isize,
    vel_y: isize,
}

#[derive(Debug, PartialEq)]
enum Miss {
    MissedShort,
    MissedLong,
    Skipped,
}

impl Probe {
    fn new(vel_x: isize, vel_y: isize) -> Probe {
        Probe {
            pos_x: 0,
            pos_y: 0,
            vel_x,
            vel_y,
        }
    }

    fn step(&mut self) {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;

        match self.vel_x.cmp(&0) {
            Ordering::Equal => {}
            Ordering::Greater => self.vel_x -= 1,
            Ordering::Less => self.vel_x += 1,
        }

        self.vel_y -= 1;
    }

    fn hits_target(&mut self, target: &TargetArea) -> Result<(isize, isize, isize), Miss> {
        let mut max_y = self.pos_y;

        loop {
            max_y = max(self.pos_y, max_y);

            if target.contains(self.pos_x, self.pos_y) {
                return Ok((self.pos_x, self.pos_y, max_y));
            }

            if self.vel_x == 0 && &self.pos_y < target.target_y.end() {
                if &self.pos_x < target.target_x.start() {
                    return Err(Miss::MissedShort);
                }

                if &self.pos_x > target.target_x.end() {
                    return Err(Miss::MissedLong);
                }

                return Err(Miss::Skipped);
            }

            self.step();
        }
    }
}

fn highest_hit(target: &TargetArea) -> (isize, isize, isize, usize) {
    let mut vel_x = 1;
    let mut vel_y = 1;
    let mut max_y = 0;
    let mut count = 0;

    for attempt_x in 1..250 {
        for attempt_y in -250..250 {
            let mut probe = Probe::new(attempt_x, attempt_y);

            if let Ok((x, y, new_max_y)) = probe.hits_target(target) {
                println!(
                    "Hit with <{},{}> at ({},{}) with highest y {}",
                    attempt_x, attempt_y, x, y, new_max_y
                );
                if new_max_y > max_y {
                    vel_x = attempt_x;
                    vel_y = attempt_y;
                    max_y = new_max_y;
                }
                count += 1;
            }
        }
    }

    (vel_x, vel_y, max_y, count)
}

fn main() {
    let test_target = TargetArea {
        target_x: 20..=30,
        target_y: -10..=-5,
    };
    let mut test_probe = Probe::new(7, 2);

    assert_eq!(Ok((28, -7, 3)), test_probe.hits_target(&test_target));
    assert_eq!((6, 9, 45, 112), highest_hit(&test_target));

    let real_target = TargetArea {
        target_x: 111..=161,
        target_y: -154..=-101,
    };
    println!(
        "target_y: {:?} (start {}, end {})",
        real_target.target_y,
        real_target.target_y.start(),
        real_target.target_y.end()
    );
    println!("Search results answer: {:?}", highest_hit(&real_target));
}
