struct Solution;

// Problem URL: https://leetcode.com/problems/binary-search/
// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
// You must write an algorithm with O(log n) runtime complexity.

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {

        if nums.len() == 0 {
            return -1;
        } else if nums.len() == 1 {
            if nums[0] == target {
                return 0;
            } else {
                return -1;
            }
        }

        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                if mid == 0 {
                    return -1;
                }
                right = mid - 1;
            }
        }
        -1
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

#[test]
fn test3() {
    assert_eq!(Solution::search(vec![2,5], 0), -1);
}