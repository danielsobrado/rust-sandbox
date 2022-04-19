struct Solution;

// Problem URL: https://leetcode.com/problems/merge-sorted-array/
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.

// The final sorted array should not be returned by the function, 
// but instead be stored inside the array nums1. 
// To accommodate this, nums1 has a length of m + n, 
// where the first m elements denote the elements that should be merged, 
// and the last n elements are set to 0 and should be ignored. 
// nums2 has a length of n.

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        0
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::merge([1,2,3,0,0,0], 3, [2,5,6], 3), [1,2,2,3,5,6]), [1,2,2,3,5,6]);
}

#[test]
fn test2() {
    assert_eq!(Solution::merge([1], 1, [], 0), [1]); ;
}

#[test]
fn test3() {
    assert_eq!(Solution::merge([0], 0, [1], 1), [1]);
}