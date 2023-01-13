use std::vec::Vec;

trait Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32;
}

struct Sol {}

impl Solution for Sol {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }

        let mut left_bound = 0usize;
        let mut right_bound = nums.len() - 1;
        let mut pos = (right_bound + left_bound) / 2;

        // Special case: the `target` is on one of the bounds

        if nums[left_bound] == target {
            return left_bound.try_into().unwrap();
        }

        if nums[right_bound] == target {
            return right_bound.try_into().unwrap();
        }

        while right_bound - left_bound > 1 {
            if target == nums[pos] {
                return pos.try_into().unwrap();
            } else if target > nums[pos] {
                left_bound = pos;
            } else {
                right_bound = pos;
            }

            pos = (right_bound + left_bound) / 2;
        }

        -1
    }
}

fn main() {
    let vecs: [Vec<i32>; 5] = [
        vec![-1, 0, 1, 2, 3],
        vec![-1, 2, 3],
        vec![-1, 3],
        vec![3],
        vec![],
    ];

    for v in &vecs {
        let pos = <Sol as Solution>::search(v.clone(), 1);
        println!("{}", pos);
    }
}
