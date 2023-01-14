// Given a sorted array of distinct integers and a target value, return the index if the target is
// found. If not, return the index where it would be if it were inserted in order.
//
// You must write an algorithm with O(log n) runtime complexity.

trait Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32;
}

struct Sol {}

#[inline(always)]
fn calculate_mean(a: i32, b: i32) -> i32 {
    (a & b) + ((a ^ b) >> 1)
}

impl Solution for Sol {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left_bound: i32 = 0;
        let mut right_bound: i32 = nums.len() as i32;
        let mut pos;

        while left_bound < right_bound {
            pos = calculate_mean(left_bound, right_bound);

            if nums[pos as usize] < target {
                left_bound = pos + 1;
            } else {
                right_bound = pos;
            }
        }

        left_bound
    }
}

fn main() {
    let vecs: [Vec<i32>; 5] = [
        vec![-1, 0, 2, 4, 6],
        vec![-1, 2, 4],
        vec![-1, 3],
        vec![3],
        vec![],
    ];
    let targets: [i32; 4] = [-1, 2, 42, 3];

    for vec in &vecs {
        for target in &targets {
            let position = Sol::search(vec.clone(), *target);
            println!("Vector={:?}, target={}, position={}", vec, target, position);
        }
    }
}
