
fn max2i32(a: i32, b: i32) -> usize {
    if a > b {
        0
    } else {
        1
    }
}

/// Queue push variant for a queue of length 1
#[inline(always)]
fn queue1_push<'a>(queue: &'a mut Vec<i32>, val: i32) -> () {
    assert!(queue.len() <= 1);

    if queue.len() < 1 {
        queue.push(val);
    } else {
        queue[0] = val;
    }
}

/// Queue push variant for a queue of length 2
#[inline(always)]
fn queue2_push<'a>(queue: &'a mut Vec<i32>, val: i32) -> () {
    assert!(queue.len() <= 2);
    queue[0] = queue[1];
    queue[1] = val;

    if queue.len() < 2 {
        queue.push(val);
    } else {
        queue[0] = val;
    }
}

type QueuePushCallback = for <'a> fn(&'a mut Vec<i32>, i32) -> ();

/// Implements a "mock" merge.
/// - We only need to find Nqueue = 1 or 2 elements from which the median is inferred;
/// - We traverse across both arrays maintaining their relative positions;
/// - On each step, a max element from 2 arrays is pushed into a FIFO queue of length `Nqueue`;
/// - We continue iterating until `stop_counter` is reached;
/// - Once the counter is reached, we calcualte an average from the FIFO queue;
/// - PROFIT!!!
fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    let resulting_length = nums1.len() + nums2.len();
    let mut arr_refs = [&mut nums1, &mut nums2];
    let mut arr_positions: [usize; 2] = [0, 0];
    let overall_len = nums1.len() + nums2.len();

    let (queue_size, stop_counter, queue_push) = {
        if overall_len & 1usize == 1usize {  // is odd
            (2usize, overall_len / 2 + 1, queue2_push as QueuePushCallback)
        } else {
            (1usize, overall_len / 2, queue1_push as QueuePushCallback)
        }
    };

    panic!();  // Not implemented
    0f64
}

fn main() -> () {
}
