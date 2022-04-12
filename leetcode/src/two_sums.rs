
struct Solution;

use std::collections::HashMap;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![]
    }
}

#[test]
fn test1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
}

#[test]
fn test2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);
}

#[test]
fn test3() {
    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
}