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

pub fn max_subarray_sum_circular(nums: &Vec<i32>) -> i32 {
    let len = nums.len();
    let mut right_max = nums.clone();

    for i in (0..right_max.len() - 1).rev() {
        right_max[i] = *[right_max[i + 1] + right_max[i], right_max[i + 1]].iter().max().unwrap();
    }

    let mut normal_max_sum = nums[0];
    let mut overlap_max_sum = nums[0];
    let mut normal_current_sum = nums[0];
    let mut overlap_suffix_sum = 0;

    for i in 0..nums.len() - 1 {
        normal_current_sum = *[nums[i + 1], normal_current_sum + nums[i + 1]]
            .iter().max().unwrap();
        normal_max_sum = *[normal_current_sum, normal_max_sum].iter().max().unwrap();
        overlap_suffix_sum += nums[i];

        if i < len - 1 {
            overlap_max_sum = *[overlap_max_sum,
                overlap_suffix_sum + right_max[i + 1]].iter().max().unwrap();
        }
    }

    *[normal_max_sum, overlap_max_sum].iter().max().unwrap()
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
