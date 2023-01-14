trait Solution {
    fn first_bad_version(&self, n: i32) -> i32;
}

struct Sol {
    first_bad: i32,
}

impl Sol {
    fn isBadVersion(&self, i: i32) -> bool {
        i >= self.first_bad && i > 0
    }
}

impl Solution for Sol {
    fn first_bad_version(&self, n: i32) -> i32 {
        let mut left_bound: i32 = 0;
        let mut right_bound: i32 = n + 1;
        let mut pos = ((left_bound as i64 + right_bound as i64) / 2) as i32;

        while left_bound <= right_bound {
            pos = ((left_bound as i64 + right_bound as i64) / 2i64) as i32;

            if self.isBadVersion(pos) && !self.isBadVersion(pos - 1) {
                return pos;
            } else if self.isBadVersion(pos) {
                right_bound = pos - 1;
            } else {
                left_bound = pos + 1;
            }
        }

        pos
    }
}

struct InputData(Sol, i32);

impl InputData {
    fn new_from_bounds(first_bad: i32, n_commits: i32) -> InputData {
        assert!(n_commits >= first_bad && first_bad > 0);

        InputData(Sol{first_bad}, n_commits)
    }
}

fn main() {
    let input_data: [InputData; 6] = [
        InputData::new_from_bounds(1, 1),
        InputData::new_from_bounds(1, 2),
        InputData::new_from_bounds(3, 3),
        InputData::new_from_bounds(2, 3),
        InputData::new_from_bounds(4, 42),
        InputData::new_from_bounds(1702766719, 2126753390),
    ];

    for input in &input_data {
        let result = input.0.first_bad_version(input.1);
        println!("Actual: {}, Found: {}", input.0.first_bad, result);
        assert!(result == input.0.first_bad);
    }
}
