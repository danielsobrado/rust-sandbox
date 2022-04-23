struct Solution;

// Problem URL: https://leetcode.com/problems/binary-search/
// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
// You must write an algorithm with O(log n) runtime complexity.

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        0
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 9), 4);
}

#[test]
fn test2() {
    assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 2), -1);
}
