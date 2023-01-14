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
        let mut pos = (left_bound + right_bound) / 2;

        while left_bound <= right_bound {
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

fn main() {
    println!("Hello, world!");
}
