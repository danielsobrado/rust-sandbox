struct Solution;

// Problem URL: https://leetcode.com/problems/rotate-array/
// Given an array, rotate the array to the right by k steps, where k is non-negative.

use std::collections::VecDeque;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {

        if nums.len() <= 1 {
            return;
        }

        let mut queue = VecDeque::<i32>::from(nums.clone());

        (0..k as usize).for_each(|_i| {
            let num = queue.pop_back().unwrap();
            queue.push_front(num);
        });

        *nums = queue.iter().map(|x| x).cloned().collect();

    }
}

#[test]
fn test1() {
    let mut nums = vec![1,2,3,4,5,6,7];
    Solution::rotate(&mut nums, 3);
    assert_eq!(nums, [5,6,7,1,2,3,4]);
}

#[test]
fn test2() {
    let mut nums = vec![-1,-100,3,99];
    Solution::rotate(&mut nums, 2);
    assert_eq!(nums, [3,99,-1,-100]);
}

#[test]
fn test3() {
    let mut nums = vec![1,2];
    Solution::rotate(&mut nums, 3);
    assert_eq!(nums, [2,1]);
}