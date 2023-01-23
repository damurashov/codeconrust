// 918
//
// Given a circular integer array nums of length n, return the maximum possible sum of a non-empty
// subarray of nums.
//
// A circular array means the end of the array connects to the beginning of the array. Formally,
// the next element of nums[i] is nums[(i + 1) % n] and the previous element of nums[i] is nums[(i
// - 1 + n) % n].
//
// A subarray may only include each element of the fixed buffer nums at most once. Formally, for a
// subarray nums[i], nums[i + 1], ..., nums[j], there does not exist i <= k1, k2 <= j with k1 % n
// == k2 % n.

use std::vec::Vec;

const MIN: i32 = -3 * 10000 - 1;

#[inline(always)]
fn as_normalized_pos(pos: i32, shift: i32, total_length: usize) -> usize {
    let ret = pos + shift;

    if ret <= 0 {
        ((total_length as i32 + (ret % total_length as i32)) % total_length as i32) as usize
    } else {
        (ret % total_length as i32) as usize
    }
}

pub fn max_subarray_sum_circular(nums: &Vec<i32>) -> i32 {
    let len = nums.len();
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    let mut current_sum = nums[0];
    let mut max_sum = current_sum;

    while ((right - left) as usize) < len - 1 && left > -(len as i32)
            && right < len as i32 {
        let step_left_candidate = current_sum + nums[as_normalized_pos(left, -1, len)];
        let step_right_candidate = current_sum + nums[as_normalized_pos(right, 1, len)];
        let override_left_candidate = nums[as_normalized_pos(left, -1, len)];
        let override_right_candidate = nums[as_normalized_pos(right, 1, len)];
        current_sum = *[step_left_candidate, step_right_candidate,
            override_left_candidate, override_right_candidate].iter().max()
            .unwrap();

        if current_sum == step_left_candidate {
            left -= 1;
        } else if current_sum == step_right_candidate {
            right += 1;
        } else if current_sum == override_left_candidate {
            left -= 1;
            right = left;
        } else if current_sum == override_right_candidate {
            right += 1;
            left = right;
        }

        if current_sum > max_sum {
            max_sum = current_sum;
        }
    }

    max_sum
}

//impl Solution {
//   pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
//       return max_subarray_sum_circular(nums);
//   }
//}

fn main() {
    let arr: [Vec<i32>; 5] = [
        vec![5, -3, 5],
        vec![-1],
        vec![-1, -2],
        vec![-1, 1, -1, 1],
        vec![-1, 2, 3, 4],
    ];

    for v in &arr {
        println!("{:?} : {}", v, max_subarray_sum_circular(v));
    }
}
