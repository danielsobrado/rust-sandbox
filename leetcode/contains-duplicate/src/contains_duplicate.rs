struct Solution;

// Problem URL: https://leetcode.com/problems/contains-duplicate/
// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        false
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::contains_duplicate(vec![1,2,3,1]), true);
}

#[test]
fn test2() {
    assert_eq!(Solution::contains_duplicate(vec![1,2,3,4]), false);
}

#[test]
fn test3() {
    assert_eq!(Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
}