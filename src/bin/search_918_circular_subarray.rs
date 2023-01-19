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

const MIN: i32 = -3 * 10000;

#[inline(always)]
fn maxi32(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn max_subarray_sum_circular(nums: &Vec<i32>) -> i32 {
    let len = nums.len();
    let mut left_pos: usize = 0;
    let mut max_sum = MIN;
    let mut current_sum = 0i32;

    for pos in 0..len * 2 {
        let normalized_pos = pos % len;

        if pos - left_pos >= len {
            let normalized_left_pos = left_pos % len;

            if current_sum + nums[normalized_pos]
                    - nums[normalized_left_pos] > nums[normalized_pos] {
                current_sum += nums[normalized_pos] - nums[left_pos % len];
            } else {
                left_pos = pos;
                current_sum = nums[normalized_pos];
            }
        } else {
            if nums[normalized_pos] + current_sum >= nums[normalized_pos] {
                current_sum += nums[normalized_pos];
            } else {
                current_sum = nums[normalized_pos];
            }
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
    let arr: [Vec<i32>; 1] = [
        vec![5, -3, 5],
    ];

    for v in &arr {
        println!("{:?} : {}", v, max_subarray_sum_circular(v));
    }
}
