// Given an array nums with n objects colored red, white, or blue, sort them
// in-place so that objects of the same color are adjacent, with the colors in
// the order red, white, and blue.
//
// We will use the integers 0, 1, and 2 to represent the color red, white, and
// blue, respectively.
//
// You must solve this problem without using the library's sort function.

use std::vec::Vec;
use std::ptr::swap;

fn hoare_partition(nums: &mut[i32]) -> i32
{
    let mut left = -1;
    let mut right = nums.len() as i32;
    let mid = left & right + ((left ^ right) >> 1);  // (a + b) / 2
    let pivot = nums[mid as usize];

    loop {
        loop {
            left += 1;

            if !(nums[left as usize] < pivot) {
                break;
            }
        }

        loop {
            right -= 1;

            if !(nums[right as usize] > pivot) {
                break;
            }
        }

        if left >= right {
            return right;
        }

        unsafe {
            swap(&mut nums[left as usize] as *mut i32, &mut nums[right as usize] as *mut i32);
        }
    }
}

trait Solution {
    fn sort_colors(nums: &mut Vec<i32>);
}

struct Sol {
}

impl Sol {
    fn sort(nums: &mut[i32], partition: fn(&mut[i32]) -> i32) {
        if nums.len() > 1 {
            let pivot = partition(nums);
            let len = nums.len();
            Self::sort(&mut nums[0..(pivot + 1) as usize], partition);
            Self::sort(&mut nums[(pivot + 1) as usize..len], partition);
        }
    }
}

impl Solution for Sol {
    fn sort_colors(nums: &mut Vec<i32>) {
        Self::sort(&mut nums[..], hoare_partition);
    }
}

fn main() {
    let mut vecs: [Vec<i32>; 3] = [
        vec![0,2,1,0,2,1,2],
        vec![0,2,1],
        vec![0],
    ];

    for i in 0..vecs.len() {
        Sol::sort_colors(&mut vecs[i]);
        println!("{:?}", &mut vecs[i]);
    }
}
