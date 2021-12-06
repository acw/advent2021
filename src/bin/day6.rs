#[cfg(test)]
const TEST_DATA: &[u8] = &[3, 4, 3, 1, 2];
const REAL_DATA: &[u8] = &[
    3, 4, 1, 2, 1, 2, 5, 1, 2, 1, 5, 4, 3, 2, 5, 1, 5, 1, 2, 2, 2, 3, 4, 5, 2, 5, 1, 3, 3, 1, 3, 4,
    1, 5, 3, 2, 2, 1, 3, 2, 5, 1, 1, 4, 1, 4, 5, 1, 3, 1, 1, 5, 3, 1, 1, 4, 2, 2, 5, 1, 5, 5, 1, 5,
    4, 1, 5, 3, 5, 1, 1, 4, 1, 2, 2, 1, 1, 1, 4, 2, 1, 3, 1, 1, 4, 5, 1, 1, 1, 1, 1, 5, 1, 1, 4, 1,
    1, 1, 1, 2, 1, 4, 2, 1, 2, 4, 1, 3, 1, 2, 3, 2, 4, 1, 1, 5, 1, 1, 1, 2, 5, 5, 1, 1, 4, 1, 2, 2,
    3, 5, 1, 4, 5, 4, 1, 3, 1, 4, 1, 4, 3, 2, 4, 3, 2, 4, 5, 1, 4, 5, 2, 1, 1, 1, 1, 1, 3, 1, 5, 1,
    3, 1, 1, 2, 1, 4, 1, 3, 1, 5, 2, 4, 2, 1, 1, 1, 2, 1, 1, 4, 1, 1, 1, 1, 1, 5, 4, 1, 3, 3, 5, 3,
    2, 5, 5, 2, 1, 5, 2, 4, 4, 1, 5, 2, 3, 1, 5, 3, 4, 1, 5, 1, 5, 3, 1, 1, 1, 4, 4, 5, 1, 1, 1, 3,
    1, 4, 5, 1, 2, 3, 1, 3, 2, 3, 1, 3, 5, 4, 3, 1, 3, 4, 3, 1, 2, 1, 1, 3, 1, 1, 3, 1, 1, 4, 1, 2,
    1, 2, 5, 1, 1, 3, 5, 3, 3, 3, 1, 1, 1, 1, 1, 5, 3, 3, 1, 1, 3, 4, 1, 1, 4, 1, 1, 2, 4, 4, 1, 1,
    3, 1, 3, 2, 2, 1, 2, 5, 3, 3, 1, 1,
];

#[derive(Debug, PartialEq)]
struct LanternFishies {
    data: [usize; 9],
}

impl<'a> From<&'a [u8]> for LanternFishies {
    fn from(state: &[u8]) -> LanternFishies {
        let mut data = [0; 9];

        for x in state.iter() {
            data[*x as usize] += 1;
        }

        LanternFishies { data }
    }
}

impl LanternFishies {
    fn count(&self) -> usize {
        self.data.iter().sum()
    }

    fn step(&mut self) {
        let base_data = self.data;

        self.data.fill(0);

        self.data[6] += base_data[0];
        self.data[8] += base_data[0];
        for (i, curval) in base_data.iter().enumerate().skip(1) {
            self.data[i - 1] += curval;
        }
    }

    fn steps(&mut self, count: usize) {
        for _ in 0..count {
            self.step();
        }
    }
}

#[test]
fn example() {
    let mut values = LanternFishies::from(TEST_DATA);
    assert_eq!(
        LanternFishies {
            data: [0, 1, 1, 2, 1, 0, 0, 0, 0]
        },
        values
    );
    assert_eq!(5, values.count());
    values.step();
    assert_eq!(
        LanternFishies {
            data: [1, 1, 2, 1, 0, 0, 0, 0, 0]
        },
        values
    );
    assert_eq!(5, values.count());
    values.step();
    assert_eq!(
        LanternFishies {
            data: [1, 2, 1, 0, 0, 0, 1, 0, 1]
        },
        values
    );
    assert_eq!(6, values.count());
    values.step();
    assert_eq!(
        LanternFishies {
            data: [2, 1, 0, 0, 0, 1, 1, 1, 1]
        },
        values
    );
    assert_eq!(7, values.count());
    values.step();
    assert_eq!(
        LanternFishies {
            data: [1, 0, 0, 0, 1, 1, 3, 1, 2]
        },
        values
    );
    assert_eq!(9, values.count());
}

#[test]
fn regression() {
    let mut test_values = LanternFishies::from(TEST_DATA);
    test_values.steps(18);
    assert_eq!(26, test_values.count());
    test_values.steps(80 - 18);
    assert_eq!(5934, test_values.count());
    test_values.steps(256 - 80);
    assert_eq!(26984457539, test_values.count());

    let mut real_values = LanternFishies::from(REAL_DATA);
    real_values.steps(80);
    assert_eq!(365131, real_values.count());
    real_values.steps(256 - 80);
    assert_eq!(1650309278600, real_values.count());
}

fn main() {
    let mut fishies = LanternFishies::from(REAL_DATA);
    fishies.steps(80);
    println!("After 80 days, there will be {} fishies.", fishies.count());
    fishies.steps(256 - 80);
    println!(
        "After 256 days, there will be {} fishies. (That's a lot of fishies!)",
        fishies.count()
    );
}
