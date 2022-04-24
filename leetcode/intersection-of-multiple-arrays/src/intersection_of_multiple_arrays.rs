struct Solution;

// Problem URL: https://leetcode.com/problems/intersection-of-multiple-arrays/
// Given a 2D integer array nums where nums[i] is a non-empty array of distinct positive integers, 
// return the list of integers that are present in each array of nums sorted in ascending order.

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::intersection(vec![vec![3,1,2,4,5], vec![1,2,3,4], vec![3,4,5,6]]), vec![3, 4]);
}

#[test]
fn test2() {
    assert_eq!(Solution::intersection(vec![vec![1, 2, 3], vec![4, 5, 6]]), vec![]);
}
